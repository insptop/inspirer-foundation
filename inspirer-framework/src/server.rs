use serde::Deserialize;
use tokio::signal;

use crate::{
    app::{AppContext, AppTrait},
    config::config_keys,
};

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    listen: String,
}

pub(crate) async fn start_server<T>(context: AppContext<T>) -> eyre::Result<()>
where
    T: AppTrait + 'static,
{
    let server_config = context.config.get::<ServerConfig>(config_keys::SERVER)?;
    let listener = tokio::net::TcpListener::bind(server_config.listen).await?;
    let routes = T::routes(context.clone())
        .await?
        .with_state(context)
        .into_make_service();

    axum::serve(listener, routes)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
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
}
