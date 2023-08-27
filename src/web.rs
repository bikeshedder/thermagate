use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade, State,
    },
    response::Response,
    routing::get,
    Router,
};
use tokio::sync::broadcast;

use crate::can::BusFrame;

pub async fn run_server(addr: SocketAddr, tx: broadcast::Sender<BusFrame>) {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws))
        .with_state(tx);
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn ws(ws: WebSocketUpgrade, tx: State<broadcast::Sender<BusFrame>>) -> Response {
    let rx = tx.subscribe();
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
