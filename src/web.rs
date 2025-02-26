use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Sse, sse},
    routing::get,
};
use internment::Intern;
use rust_embed::Embed;
use serde::Serialize;
use socketcan::EmbeddedFrame;
use socketioxide::{SocketIo, extract::SocketRef};
use tokio::sync::broadcast;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};
use tower_http::cors::CorsLayer;
use tracing::{info, warn};

use crate::{
    can::{
        self,
        driver::{CanDriver, ReceivedMessage},
        message::MessageType,
    },
    catalog::{Catalog, param::Param as CatalogParam},
    commands::gateway::Params,
    config::HttpConfig,
    model::value::Value,
    utils::warn_if_err,
};

#[derive(Embed)]
#[folder = "frontend/dist"]
struct Assets;

#[derive(Clone)]
struct AppState {
    pub catalog: Arc<Catalog>,
    pub params: Params,
    pub can: Arc<CanDriver>,
}

pub async fn run_server(
    config: &HttpConfig,
    catalog: Arc<Catalog>,
    params_data: Params,
    can: Arc<CanDriver>,
) {
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

    tokio::spawn(can_sender(catalog.clone(), can.rx(), io.clone()));
    tokio::spawn(param_sender(params_data.tx.subscribe(), io.clone()));

    let app = Router::new() //
        .route("/api/v1/master_data", get(get_master_data))
        .route("/api/v1/params", get(params))
        .route("/api/v1/can_monitor", get(can_monitor))
        .with_state(AppState {
            catalog: catalog.clone(),
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
    pub name: Option<Intern<String>>,
}

#[derive(Debug, Serialize)]
pub struct Param {
    pub id: u16,
    pub name: Intern<String>,
}

impl From<&CatalogParam> for Param {
    fn from(value: &CatalogParam) -> Self {
        Self {
            id: value.id,
            name: value.name,
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
    value: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct ParamUpdate {
    pub device: Device,
    pub param: Param,
    pub value: crate::commands::gateway::Param,
}

async fn can_sender(
    catalog: Arc<Catalog>,
    mut rx: broadcast::Receiver<ReceivedMessage>,
    io: SocketIo,
) {
    while let Ok(msg) = rx.recv().await {
        let p_v = msg.decode_param(&catalog);
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
                        name: p_v.as_ref().map(|(p, _)| p.name),
                    },
                    value: if matches!(msg.r#type, MessageType::Response | MessageType::Set) {
                        p_v.and_then(|(_, v)| v)
                    } else {
                        None
                    },
                },
            )
            .await,
            "io.emit(\"can\", ...) failed",
        );
    }
}

async fn param_sender(mut rx: broadcast::Receiver<Arc<ParamUpdate>>, io: SocketIo) {
    while let Ok(update) = rx.recv().await {
        warn_if_err(
            io.emit("param", &update).await,
            "io.emit(\"param\", ...) failed",
        );
    }
}

#[derive(Debug, Serialize)]
struct MasterData {
    params: Vec<MasterDataParam>,
}

#[derive(Debug, Serialize)]
struct MasterDataParam {
    name: Intern<String>,
}

async fn get_master_data(state: State<AppState>) -> impl IntoResponse {
    // XXX add caching
    let config = MasterData {
        params: state
            .catalog
            .params
            .iter()
            .map(|param| MasterDataParam { name: param.name })
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
