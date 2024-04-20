//! About framework response message

use crate::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
/// Structure representing details about an error.
pub struct ErrorDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ErrorDetail {
    /// Create a new `ErrorDetail` with the specified error and description.
    #[must_use]
    pub fn new<T: Into<String>>(error: T, description: T) -> Self {
        Self {
            error: Some(error.into()),
            description: Some(description.into()),
        }
    }

    /// Create an `ErrorDetail` with only an error reason and no description.
    #[must_use]
    pub fn with_reason<T: Into<String>>(error: T) -> Self {
        Self {
            error: Some(error.into()),
            description: None,
        }
    }
}

pub type Resp<T> = crate::Result<Json<ResponseMessage<T>>>;

/// Default json response message
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage<T = ()>
where
    T: Serialize,
{
    /// 成功状态
    pub success: bool,
    /// 返回数据
    pub data: T,
    /// 行为（该字段可能不存在）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviour: Option<()>,
}

pub fn json_response<T: Serialize>(data: T) -> Json<ResponseMessage<T>> {
    Json(ResponseMessage {
        success: true,
        data,
        behaviour: None,
    })
}

pub fn json_error_response<T: Serialize>(data: T) -> Json<ResponseMessage<T>> {
    Json(ResponseMessage {
        success: false,
        data,
        behaviour: None,
    })
}

/// Return a success message use default response message, See [`ResponseMessage`].
pub fn ok<T: Serialize>(data: T) -> Resp<T> {
    Ok(json_response(data))
}

impl IntoResponse for Error {
    /// Convert an `Error` into an HTTP response.
    fn into_response(self) -> Response {
        match &self {
            Self::WithBacktrace {
                inner,
                backtrace: _,
            } => {
                tracing::error!(
                error.msg = %inner,
                error.details = ?inner,
                "controller_error"
                );
            }
            err => {
                tracing::error!(
                error.msg = %err,
                error.details = ?err,
                "controller_error"
                );
            }
        }

        let public_facing_error = match self {
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorDetail::new("not_found", "Resource was not found"),
            ),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorDetail::new("internal_server_error", "Internal Server Error"),
            ),
            Self::Unauthorized(err) => {
                tracing::warn!(err);
                (
                    StatusCode::UNAUTHORIZED,
                    ErrorDetail::new(
                        "unauthorized",
                        "You do not have permission to access this resource",
                    ),
                )
            }
            Self::CustomError(status_code, data) => (status_code, data),
            Self::WithBacktrace { inner, backtrace } => {
                println!("\n{}", inner.to_string().red().underline());
                backtrace::print_backtrace(&backtrace).unwrap();
                (
                    StatusCode::BAD_REQUEST,
                    ErrorDetail::with_reason("Bad Request"),
                )
            }
            _ => (
                StatusCode::BAD_REQUEST,
                ErrorDetail::with_reason("Bad Request"),
            ),
        };

        (
            public_facing_error.0,
            json_error_response(public_facing_error.1),
        )
            .into_response()
    }
}

mod backtrace {
    use crate::{Error, Result};
    use once_cell::sync::Lazy;
    use regex::Regex;
    static NAME_BLOCKLIST: Lazy<Vec<Regex>> = Lazy::new(|| {
        [
            "^___rust_try",
            "^__pthread",
            "^__clone",
            "^<loco_rs::errors::Error as",
            "^loco_rs::errors::Error::bt",
            /*
            "^<?tokio",
            "^<?future",
            "^<?tower",
            "^<?futures",
            "^<?hyper",
            "^<?axum",
            "<F as futures_core",
            "^<F as axum::",
            "^<?std::panic",
            "^<?core::",
            "^rust_panic",
            "^rayon",
            "^rust_begin_unwind",
            "^start_thread",
            "^call_once",
            "^catch_unwind",
            */
        ]
        .iter()
        .map(|s| Regex::new(s).unwrap())
        .collect::<Vec<_>>()
    });

    static FILE_BLOCKLIST: Lazy<Vec<Regex>> = Lazy::new(|| {
        [
            "axum-.*$",
            "tower-.*$",
            "hyper-.*$",
            "tokio-.*$",
            "futures-.*$",
            "^/rustc",
        ]
        .iter()
        .map(|s| Regex::new(s).unwrap())
        .collect::<Vec<_>>()
    });

    pub fn print_backtrace(bt: &std::backtrace::Backtrace) -> Result<()> {
        backtrace_printer::print_backtrace(
            &mut std::io::stdout(),
            bt,
            &NAME_BLOCKLIST,
            &FILE_BLOCKLIST,
        )
        .map_err(Error::msg)
    }
}
