#[cfg(feature = "enable-axum")]
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::fmt;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct InspirerWebApplicationError(StatusCode, u32, &'static str);

impl std::error::Error for InspirerWebApplicationError {}

impl fmt::Display for InspirerWebApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error({}): {}", self.1, self.2)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Extract Service extension error.")]
    ExtractServiceExtensionFailed,
    #[error("Get configuration data failed.")]
    GetConfigurationFailedError,
    #[error("Get configuration component failed.")]
    GetConfigurationComponentFailed,
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    InspirerWebApplicationError(#[from] InspirerWebApplicationError),
}

#[derive(Debug, Serialize)]
struct ErrorMessage<T = ()>
where
    T: Serialize,
{
    code: u32,
    msg: String,
    data: T,
}

#[cfg(feature = "enable-axum")]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let msg = format!("{}", self);
        let (code, status) = match self {
            Self::InspirerWebApplicationError(InspirerWebApplicationError(status, code, _)) => {
                (code, status)
            }
            Self::ExtractServiceExtensionFailed => (1, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationFailedError => (2, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationComponentFailed => (3, StatusCode::INTERNAL_SERVER_ERROR),
            Self::ConfigError(_) => (4, StatusCode::INTERNAL_SERVER_ERROR),
            Self::DatabaseError(_) => (5, StatusCode::INTERNAL_SERVER_ERROR),
        };

        (
            status,
            Json(ErrorMessage {
                code,
                msg,
                data: Option::<()>::None,
            }),
        )
            .into_response()
    }
}
