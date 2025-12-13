use axum_client_ip::ClientIpSource;
use config::Config;
use serde::Deserialize;

use crate::error::AppError;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub ip_source: ClientIpSource,
    pub port: u16,
}

impl AppConfig {
    pub fn new() -> Result<AppConfig, AppError> {
        let config = Config::builder()
            .set_default("ip_source", "ConnectInfo")?
            .set_default("port", 8080)?
            .add_source(config::Environment::with_prefix("WHOAMI"))
            .build()?;

        config
            .try_deserialize::<AppConfig>()
            .map_err(AppError::ConfigError)
    }
}
