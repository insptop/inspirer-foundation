use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::Result;
use axum::Router;
use serde::{Deserialize, Serialize};
use tokio::signal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ServerConfig {
    pub listen: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            listen: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8088)),
        }
    }
}

/// 启动服务器
pub async fn start_server(listen: &SocketAddr, router: Router) -> Result<()> {
    log::info!("Start server.");
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

        log::debug!("Received termiate signal.");
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
