use std::net::SocketAddr;

use axum::serve;
use ifconfig_me::{config::AppConfig, router::setup_router, tracing::setup_tracing};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    setup_tracing()?;

    let app_config = AppConfig::new()?;
    let address = format!("0.0.0.0:{}", app_config.port);
    info!("Listening on {}", address);

    let router = setup_router(app_config);
    let listener = TcpListener::bind(address).await?;

    serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}
