use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::routes::main_router;

pub async fn run_server() {
    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, main_router::main_router()).await {
        Ok(_) => println!("Server is Running"),
        Err(e) => eprintln!("Error Runnignn Server:{}", e),
    }
}
