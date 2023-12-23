use std::{net::SocketAddr, sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Method,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use maud::{html, DOCTYPE};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    can::driver::{CanDriver, ReceivedMessage},
    commands::gateway::Params,
};

#[derive(Clone)]
struct AppState {
    pub params: Params,
    pub can: Arc<CanDriver>,
}

pub async fn run_server(addr: SocketAddr, params_data: Params, can: Arc<CanDriver>) {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws))
        .route("/params", get(params))
        .layer(cors)
        .with_state(AppState {
            params: params_data,
            can,
        });
    tracing::debug!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    html! {
        (DOCTYPE)
        html {
            head {

            }
            body {
                h1 {
                    "Hello world!"
                }
            }
        }
    }
}

async fn params(state: State<AppState>) -> impl IntoResponse {
    let values = state.params.0.lock().await.clone();
    Json(values)
}

async fn ws(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    let rx = state.0.can.rx();
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<ReceivedMessage>) {
    loop {
        let Ok(message) = rx.recv().await else {
            // Server is shutting down
            return;
        };
        if socket
            .send(WsMessage::Text(serde_json::to_string(&*message).unwrap()))
            .await
            .is_err()
        {
            return;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
