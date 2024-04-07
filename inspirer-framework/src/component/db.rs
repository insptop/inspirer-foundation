use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DbConn};
use serde::{Deserialize, Serialize};

use super::ComponentProvider;

#[derive(Deserialize, Serialize, Debug)]
pub struct ComponentConfig {
    // The URI for connecting to the database. For example:
    /// * Postgres: `postgres://root:12341234@localhost:5432/myapp_development`
    /// * Sqlite: `sqlite://db.sqlite?mode=rwc`
    pub uri: String,

    /// Enable SQLx statement logging
    #[serde(default)]
    pub enable_logging: bool,

    /// Minimum number of connections for a pool
    pub min_connections: Option<u32>,

    /// Maximum number of connections for a pool
    pub max_connections: Option<u32>,

    /// Set the timeout duration when acquiring a connection
    pub connect_timeout: Option<u64>,

    /// Set the idle duration before closing a connection
    pub idle_timeout: Option<u64>,
}

pub async fn create_component(config: ComponentConfig) -> Result<DbConn, sea_orm::DbErr> {
    let mut opt = ConnectOptions::new(&config.uri);

    tracing::debug!("connect database to {}", config.uri);

    if let Some(max_connections) = config.max_connections {
        opt.max_connections(max_connections);
    }
    if let Some(min_connections) = config.min_connections {
        opt.min_connections(min_connections);
    }
    if let Some(connect_timeout) = config.connect_timeout {
        opt.connect_timeout(Duration::from_millis(connect_timeout));
    }
    if let Some(idle_timeout) = config.idle_timeout {
        opt.idle_timeout(Duration::from_millis(idle_timeout));
    }

    opt.sqlx_logging(config.enable_logging);

    Database::connect(opt).await
}

#[async_trait::async_trait]
impl ComponentProvider for DbConn {
    type Error = sea_orm::DbErr;
    type Config = ComponentConfig;

    fn config_key() -> &'static str {
        "database"
    }

    async fn create(config: Self::Config) -> Result<Self, Self::Error> {
        create_component(config).await
    }
}
