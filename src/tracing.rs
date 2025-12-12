use tracing_subscriber::{EnvFilter, Registry, fmt::layer, layer::SubscriberExt};

use crate::error::AppError;

pub fn setup_tracing() -> Result<(), AppError> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = Registry::default().with(env_filter).with(layer().json());

    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}
