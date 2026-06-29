use std::net::SocketAddr;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use tokio::net::TcpListener;

async fn health() -> Json<Value> {
    Json(json!(
        {
            "status":"ok",
            "running" : "Fine"
        }
    ))
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, Router::new().route("/health", get(health)))
        .await
        .unwrap();
}
