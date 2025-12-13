use std::net::SocketAddr;

use axum::serve;
use ifconfig_me::{config::AppConfig, router::setup_router, tracing::setup_tracing};
use tokio::{net::TcpListener, signal};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_tracing()?;

    let app_config = AppConfig::new()?;
    let address = format!("[::]:{}", app_config.port);
    info!("Listening on {}", address);

    let router = setup_router(app_config);
    let listener = TcpListener::bind(address).await?;

    serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+c handler");
    };

    #[cfg(unix)] // This covers Linux (Podman environment)
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    // Wait for either Ctrl+C (SIGINT) or SIGTERM
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("🛑 Shutdown signal received, starting graceful shutdown...");
}
