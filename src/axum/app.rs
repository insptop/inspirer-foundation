use std::net::SocketAddr;

use axum::Router;
use crate::Result;

use super::server::start_server;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        start_server(&"0.0.0.0:8088".parse().unwrap(), self.router.clone()).await
    }
}