use std::sync::Arc;

use axum::{
    extract::State,
    http::{header, StatusCode, Uri},
    response::{sse, Html, IntoResponse, Sse},
    routing::get,
    Json, Router,
};
use rust_embed::Embed;
use serde::Serialize;
use socketcan::EmbeddedFrame;
use socketioxide::{extract::SocketRef, SocketIo};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

use crate::{
    can::{
        self,
        driver::{CanDriver, ReceivedMessage},
        message::MessageType,
        param::AnyValue,
        params::PARAMS,
    },
    commands::gateway::Params,
    config::HttpConfig,
    utils::warn_if_err,
};

#[derive(Embed)]
#[folder = "frontend/dist"]
struct Assets;

#[derive(Clone)]
struct AppState {
    pub params: Params,
    pub can: Arc<CanDriver>,
}

pub async fn run_server(config: &HttpConfig, params_data: Params, can: Arc<CanDriver>) {
    let (io_svc, io) = SocketIo::new_svc();
    io.ns("/", {
        let params_data = params_data.clone();
        |socket: SocketRef| async move {
            info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
            let params = params_data.values.lock().await;
            if let Err(e) = socket.emit("params", &*params) {
                warn!("socket.emit failed for params: {}", e);
            }
        }
    });

    tokio::spawn(can_sender(can.rx(), io.clone()));
    tokio::spawn(param_sender(params_data.tx.subscribe(), io.clone()));

    let app = Router::new() //
        .route("/api/v1/master_data", get(get_master_data))
        .route("/api/v1/params", get(params))
        .route("/api/v1/can_monitor", get(can_monitor))
        .with_state(AppState {
            can: can.clone(),
            params: params_data,
        })
        .route_service("/socket.io/", io_svc)
        .layer(CorsLayer::permissive())
        .fallback(static_handler);
    tracing::debug!("listening on http://{}", config.listen);
    let listener = tokio::net::TcpListener::bind(config.listen).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

const ASSET_EXTENSIONS: &[&str] = &["css", "jpg", "js", "png", "webmanifest", "woff2"];

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');
    match path.rsplit_once(".") {
        Some((_, ext)) if ASSET_EXTENSIONS.contains(&ext) => {
            if let Some(content) = Assets::get(path) {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            } else {
                (StatusCode::NOT_FOUND, "404").into_response()
            }
        }
        _ => {
            let content = Assets::get("index.html").unwrap();
            Html(content.data).into_response()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Device {
    pub id: u16,
    pub name: &'static str,
}

impl From<can::device::Device> for Device {
    fn from(value: can::device::Device) -> Self {
        Self {
            id: value.id(),
            name: value.name(),
        }
    }
}

#[derive(Serialize)]
pub struct CanParam {
    pub id: u16,
    pub name: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub id: u16,
    pub name: &'static str,
}

impl From<&dyn can::param::Param> for Param {
    fn from(value: &dyn can::param::Param) -> Self {
        Self {
            id: value.id(),
            name: value.name(),
        }
    }
}

#[derive(Serialize)]
struct CanMessage<'a> {
    raw: &'a [u8],
    sender: Device,
    receiver: Device,
    r#type: MessageType,
    param: CanParam,
    value: Option<AnyValue>,
}

#[derive(Debug, Serialize)]
pub struct ParamUpdate {
    pub device: Device,
    pub param: Param,
    pub value: crate::commands::gateway::Param,
}

async fn can_sender(mut rx: broadcast::Receiver<ReceivedMessage>, io: SocketIo) {
    while let Ok(msg) = rx.recv().await {
        let p_v = msg.decode_param();
        warn_if_err(
            io.emit(
                "can",
                &CanMessage {
                    raw: msg.frame.data(),
                    sender: Device::from(msg.sender),
                    receiver: Device::from(msg.receiver),
                    r#type: msg.r#type,
                    param: CanParam {
                        id: msg.param,
                        name: p_v.as_ref().map(|(p, _)| p.name()),
                    },
                    value: if matches!(msg.r#type, MessageType::Response | MessageType::Set) {
                        p_v.and_then(|(_, v)| v)
                    } else {
                        None
                    },
                },
            ),
            "io.emit(\"can\", ...) failed",
        );
    }
}

async fn param_sender(mut rx: broadcast::Receiver<Arc<ParamUpdate>>, io: SocketIo) {
    while let Ok(update) = rx.recv().await {
        warn_if_err(io.emit("param", &update), "io.emit(\"param\", ...) failed");
    }
}

#[derive(Debug, Serialize)]
struct MasterData {
    params: Vec<MasterDataParam>,
}

#[derive(Debug, Serialize)]
struct MasterDataParam {
    name: String,
}

async fn get_master_data(state: State<AppState>) -> impl IntoResponse {
    let config = MasterData {
        params: PARAMS
            .values()
            .map(|param| MasterDataParam {
                name: param.name().into(),
            })
            .collect(),
    };
    Json(config)
}

async fn params(state: State<AppState>) -> impl IntoResponse {
    let values = state.params.values.lock().await.clone();
    Json(values)
}

async fn can_monitor(state: State<AppState>) -> impl IntoResponse {
    let stream = BroadcastStream::new(state.can.rx())
        .map(|msg| msg.map(|m| sse::Event::default().json_data(*m).unwrap()));
    Sse::new(stream)
}
