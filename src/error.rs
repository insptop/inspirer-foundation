#[cfg(feature = "enable-axum")]
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct InspirerWebApplicationError(pub StatusCode, pub u32, pub &'static str);

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
    #[error(transparent)]
    ValidateError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),
    #[error("System internal error.")]
    UnknownError,
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
            Self::UnknownError => (1, StatusCode::INTERNAL_SERVER_ERROR),
            Self::InspirerWebApplicationError(InspirerWebApplicationError(status, code, _)) => {
                (code, status)
            }
            Self::ExtractServiceExtensionFailed => (2, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationFailedError => (3, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationComponentFailed => (4, StatusCode::INTERNAL_SERVER_ERROR),
            Self::ConfigError(_) => (5, StatusCode::INTERNAL_SERVER_ERROR),
            Self::DatabaseError(_) => (6, StatusCode::INTERNAL_SERVER_ERROR),
            Self::AxumFormRejection(_)
            | Self::AxumJsonRejection(_)
            | Self::AxumQueryRejection(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorMessage {
                        code: 7,
                        msg: "请求参数错误".into(),
                        data: Option::<()>::None,
                    }),
                )
                    .into_response()
            }
            Self::ValidateError(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorMessage {
                        code: 8,
                        msg: "请求参数错误".into(),
                        data: err.errors(),
                    }),
                )
                    .into_response()
            }
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
