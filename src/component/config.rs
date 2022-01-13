use std::sync::Arc;

use crate::{service::Service, Error, Result};
use config::{Config as LocalRepository, Source, Value};
use serde::Deserialize;
use tokio::sync::RwLock;

use super::ComponentConstructor;

pub struct ConfigComponentSimpleConstructor;

#[async_trait]
impl ComponentConstructor for ConfigComponentSimpleConstructor {
    async fn constructor(&self, service: Service) -> Result<()> {
        service.register_component(Config::default()).await;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    inner: Arc<RwLock<LocalRepository>>,
}

#[async_trait]
pub trait ConfigAdapter {
    async fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T>;

    async fn merge<T>(&self, source: T) -> Result<()>
    where
        T: 'static,
        T: Source + Send + Sync;

    async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value> + Send;
}

#[async_trait]
impl ConfigAdapter for Config {
    async fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        self.inner.read().await.get(key).map_err(Into::into)
    }

    async fn merge<T>(&self, source: T) -> Result<()>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        {
            self.inner.write().await.merge(source)?;
        }

        Ok(())
    }

    async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value> + Send,
    {
        {
            self.inner.write().await.set(key, value)?;
        }

        Ok(())
    }
}

#[async_trait]
impl ConfigAdapter for Service {
    async fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T> {
        self.component_guard::<Config>().await.get::<T>(key).await
    }

    async fn merge<T>(&self, source: T) -> Result<()>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        self.component_guard::<Config>().await.merge(source).await
    }

    async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value> + Send,
    {
        self.component_guard::<Config>().await.set(key, value).await
    }
}
