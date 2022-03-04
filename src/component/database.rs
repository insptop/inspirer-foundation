use super::ComponentProvider;
use crate::{Error, Result};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::ops::Deref;

pub use sea_orm::ConnectionTrait;

use super::config::{Config, ConfigAdapter};
use super::ComponentConstructor;

pub async fn create_connection(database_url: &str) -> Result<DatabaseConnection> {
    Database::connect(database_url).await.map_err(Into::into)
}

pub struct DatabaseComponentConstructor;

#[async_trait]
impl ComponentConstructor for DatabaseComponentConstructor {
    async fn constructor(&self, service: ComponentProvider) -> Result<()> {
        let database_url = match service.try_get_component::<Config>().await {
            Some(config) => config
                .try_get::<String>("database.connection_url")
                .await
                .map(|res| res.or_else(|| env::var("DATABASE_URL").ok()))?,
            None => env::var("DATABASE_URL").ok(),
        };

        if let Some(database_url) = database_url {
            service
                .register_component(create_connection(database_url.as_str()).await?)
                .await;

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
impl DaoService for ComponentProvider {
    async fn database_connection(&self) -> DatabaseConnection {
        self.component::<DatabaseConnection>().await
    }
}

pub struct Dao<'a, C: ConnectionTrait>(pub &'a C);

impl<'a, C> Deref for Dao<'a, C>
where
    C: ConnectionTrait,
{
    type Target = C;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
