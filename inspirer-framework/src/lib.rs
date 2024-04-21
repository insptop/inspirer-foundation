//! Inspirer framework
//!
//! The framework base on [`axum`], and provide more components to simplify development.

pub mod app;
pub mod cli;
pub mod command;
pub mod component;
pub mod config;
pub mod error;
pub mod logger;
pub mod response;
pub mod server;

pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub mod preludes {
    pub use crate::app::AppContext;
    pub use crate::app::AppTrait;
    pub use crate::app::Booter;
    pub use crate::command::{AppCommand, CommandRegister};
    pub use crate::error::Error;
    pub use crate::response::{ok, Resp, ResponseMessage};
    pub use crate::Result;
    pub use axum::{http::StatusCode, response::IntoResponse, Json, Router as AxumRouter};
    pub type Router<T> = axum::Router<AppContext<T>>;
}

pub use axum::{self, extract, http, routing};
pub use tower;
pub use tower_http;
