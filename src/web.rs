use std::net::SocketAddr;

use axum::{routing::get, Router};

pub async fn run_server(addr: SocketAddr) {
    let app = Router::new().route("/", get(root));
    tracing::debug!("listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
