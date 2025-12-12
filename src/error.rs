use std::{
    error::Error,
    fmt::{Debug, Formatter},
};

use axum::{http::StatusCode, response::IntoResponse};
use config::ConfigError;
use tracing::{error, subscriber::SetGlobalDefaultError};

#[derive(thiserror::Error)]
pub enum AppError {
    #[error("error when setup tracing: {0}")]
    TracingError(
        #[source]
        #[from]
        SetGlobalDefaultError,
    ),

    #[error("error when read config: {0}")]
    ConfigError(
        #[source]
        #[from]
        ConfigError,
    ),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        error!("error: {:?}", self);

        match self {
            AppError::TracingError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            AppError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl Debug for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(e: &impl Error, f: &mut Formatter<'_>) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;

    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
