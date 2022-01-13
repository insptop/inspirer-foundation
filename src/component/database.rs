use sea_orm::{DatabaseConnection, Database};
use std::env;
use crate::{Result, Error};
use crate::service::Service;

pub use sea_orm::ConnectionTrait;

use super::ComponentConstructor;
use super::config::{Config, ConfigAdapter};

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection> {
    Database::connect(database_url).await.map_err(Into::into)
}

pub struct DatabaseComponentConstructor;

#[async_trait]
impl ComponentConstructor for DatabaseComponentConstructor {
    async fn constructor(&self, service: Service) -> Result<()> {
        log::debug!("Component <DatabaseConnection> creating.");

        let database_url = match service.try_get_component::<Config>().await {
            Some(config) => config
                .get::<Option<String>>("database.connection_url")
                .await
                .and_then(|res| Ok(res.or(env::var("DATABASE_URL").ok())))?,
            None => env::var("DATABASE_URL").ok(),
        };
    
        if let Some(database_url) = database_url {
            service.register_component(create_connection(database_url.as_str()).await?).await;

            Ok(())
        } else {
            log::error!("Get connection configuration field failed.");
            Err(Error::GetConfigurationFailedError)
        }
    }
}

#[async_trait]
pub trait DaoService {
    async fn database_connection(&self) -> DatabaseConnection;
}

#[async_trait]
impl DaoService for Service {
    async fn database_connection(&self) -> DatabaseConnection {
        self.component::<DatabaseConnection>().await
    }
}

pub struct Dao<'a, C: ConnectionTrait<'a>>(pub &'a C);