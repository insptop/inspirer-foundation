pub mod app;
pub mod cli;
pub mod component;
pub mod config;
pub mod error;
pub mod logger;
pub mod response;
pub mod server;
pub mod command;

pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub mod preludes {
    pub use crate::error::Error;
    pub use crate::Result;
    pub use crate::app::AppTrait;
    pub use crate::app::Booter;
    pub use crate::app::AppContext;
    pub use crate::command::{AppCommand, CommandRegister};
    pub use crate::response::{Resp, ResponseMessage, ok};
}