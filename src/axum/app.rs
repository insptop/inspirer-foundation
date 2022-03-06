use std::net::SocketAddr;

use axum::Router;
use crate::Result;

pub struct App {
    pub router: Router,
}

impl App {
    pub async fn run(&self) -> Result<()> {
        start_server(&"0.0.0.0:8088".parse().unwrap(), self.router.clone()).await
    }
}

async fn start_server(listen: &SocketAddr, router: Router) -> Result<()> {
    axum::Server::bind(listen)
        .serve(router.into_make_service())
        .await
        .map_err(Into::into)
}