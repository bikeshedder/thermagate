use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::Method,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use tokio::sync::broadcast;
use tower_http::cors::{Any, CorsLayer};

use crate::{can::BusFrame, commands::gateway::Params};

#[derive(Clone)]
struct AppState {
    pub params: Params,
    pub tx: broadcast::Sender<BusFrame>,
}

pub async fn run_server(addr: SocketAddr, params_data: Params, tx: broadcast::Sender<BusFrame>) {
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
            tx,
        });
    tracing::debug!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn params(state: State<AppState>) -> impl IntoResponse {
    let values = state.params.0.lock().await.clone();
    Json(values)
}

async fn ws(ws: WebSocketUpgrade, state: State<AppState>) -> Response {
    let rx = state.tx.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<BusFrame>) {
    loop {
        let Ok(frame) = rx.recv().await else {
            // Server is shutting down
            return;
        };
        if socket
            .send(Message::Text(serde_json::to_string(&frame).unwrap()))
            .await
            .is_err()
        {
            return;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
