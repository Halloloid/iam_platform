use tokio::signal;

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    };
 
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to listen for SIGTERM")
            .recv()
            .await;
    };
 
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
 
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
 
    tracing::info!("Shutting down gracefully...");
}