#[cfg(feature = "enable-axum")]
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::fmt::{self, Debug, Display};

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// 错误消息
#[derive(Debug, Serialize)]
pub struct ErrorMessage<S, T = ()>
where
    S: Display + Debug,
    T: Serialize + Debug,
{
    #[serde(skip)]
    pub status: StatusCode,
    pub code: u32,
    pub msg: S,
    pub data: T,
}

/// 错误消息模板
pub type ErrorMessageTemplate = ErrorMessage<&'static str, ()>;

impl<S, T> std::error::Error for ErrorMessage<S, T>
where
    S: Serialize + Display + Debug,
    T: Serialize + Debug,
{
}

impl<S, T> fmt::Display for ErrorMessage<S, T>
where
    S: Serialize + Display + Debug,
    T: Serialize + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<S, T> IntoResponse for ErrorMessage<S, T>
where
    S: Serialize + Display + Debug,
    T: Serialize + Debug,
{
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

#[macro_export]
macro_rules! define_inspirer_error {
    ($name:ident, $status:expr, $code:literal, $msg:literal) => {
        pub const $name: $crate::ErrorMessageTemplate = ErrorMessageTemplate { status: $status, code: $code, msg: $msg, data: () };
    };
}

define_inspirer_error!(UNKONWN_ERROR, StatusCode::INTERNAL_SERVER_ERROR, 1, "System internal error.");

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "app-ext")]
    #[error(transparent)]
    LibraryLoadingError(#[from] libloading::Error),
    #[cfg(feature = "app-ext")]
    #[error("Error to load application.")]
    LoadApplicationError,
    #[error(transparent)]
    HyperError(#[from] hyper::Error),
    #[error("Runtime build error: {0}")]
    RuntimeBuildError(#[source] std::io::Error),
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
    InspirerWebApplicationErrorMessage(#[from] ErrorMessageTemplate),
    #[error(transparent)]
    ValidateError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),
    #[error("Resource not found.")]
    ResourceNotFound,
    #[error("System internal error.")]
    UnknownError,
}

#[cfg(feature = "enable-axum")]
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let (code, status) = match self {
            Self::UnknownError => (1, StatusCode::INTERNAL_SERVER_ERROR),
            Self::ExtractServiceExtensionFailed => (2, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationFailedError => (3, StatusCode::INTERNAL_SERVER_ERROR),
            Self::GetConfigurationComponentFailed => (4, StatusCode::INTERNAL_SERVER_ERROR),
            Self::ConfigError(_) => (5, StatusCode::INTERNAL_SERVER_ERROR),
            Self::DatabaseError(_) => (6, StatusCode::INTERNAL_SERVER_ERROR),
            Self::AxumFormRejection(_)
            | Self::AxumJsonRejection(_)
            | Self::AxumQueryRejection(_) => {
                return ErrorMessage {
                    code: 7,
                    msg: "请求参数解析错误",
                    data: Option::<()>::None,
                    status: StatusCode::BAD_REQUEST,
                }
                .into_response();
            }
            Self::ValidateError(err) => {
                return ErrorMessage {
                    code: 8,
                    msg: "请求参数错误",
                    data: err.errors(),
                    status: StatusCode::BAD_REQUEST,
                }
                .into_response();
            }
            Self::InspirerWebApplicationErrorMessage(msg) => return msg.into_response(),
            Self::ResourceNotFound => (9, StatusCode::NOT_FOUND),
            _ => (1, StatusCode::INTERNAL_SERVER_ERROR),
        };

        ErrorMessage {
            status,
            code,
            msg: format!("{}", self),
            data: Option::<()>::None,
        }
        .into_response()
    }
}
