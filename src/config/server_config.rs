use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::{config::gracefull_shutdown::shutdown_signal, routes::main_router};

pub async fn run_server(pool: Pool<Postgres>) {
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(
        listener,
        main_router::main_router(pool).into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    {
        eprintln!("Error Runnignn Server:{}", e);
    }
}
