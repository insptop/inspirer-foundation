use std::net::SocketAddr;

use axum::Router;
use serde::Deserialize;
use tokio::signal;
use crate::Result;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub listen: SocketAddr,
}

/// 启动服务器
pub async fn start_server(listen: &SocketAddr, router: Router) -> Result<()> {
    axum::Server::bind(listen)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(Into::into)
}


async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}