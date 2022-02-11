use std::sync::Arc;

use crate::{service::Service, Result};
use config::{Config as LocalRepository, ConfigError, Value};
use serde::Deserialize;
use tokio::sync::RwLock;

use super::ComponentConstructor;

pub use config::{Source, File, FileSourceFile, FileSourceString, Environment};

pub struct ConfigComponentSimpleConstructor;

#[async_trait]
impl ComponentConstructor for ConfigComponentSimpleConstructor {
    async fn constructor(&self, service: Service) -> Result<()> {
        service.register_component(Config::default()).await;

        Ok(())
    }
}

pub struct ConfigComponentConstructor<T: Source + Send + Sync>(pub T);

#[async_trait]
impl<T> ComponentConstructor for ConfigComponentConstructor<T>
where
    T: Source + Send + Sync + 'static,
{
    async fn constructor(&self, service: Service) -> Result<()> {
        service
            .register_component(Config::from(
                LocalRepository::new().with_merged(vec![self.0.clone_into_box()])?,
            ))
            .await;

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    inner: Arc<RwLock<LocalRepository>>,
}

impl From<LocalRepository> for Config {
    fn from(config: LocalRepository) -> Self {
        Config {
            inner: Arc::new(RwLock::new(config)),
        }
    }
}

#[async_trait]
pub trait ConfigAdapter {
    async fn get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<T>;
    async fn try_get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<Option<T>>;

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

    async fn try_get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        self.inner
            .read()
            .await
            .get(key)
            .map(|s| Some(s))
            .or_else(|err| match err {
                ConfigError::NotFound(_) => Ok(None),
                _ => Err(err),
            })
            .map_err(Into::into)
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
        self.component_read_guard::<Config>()
            .await
            .get::<T>(key)
            .await
    }

    async fn try_get<'de, T: Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        self.component_read_guard::<Config>()
            .await
            .try_get::<T>(key)
            .await
    }

    async fn merge<T>(&self, source: T) -> Result<()>
    where
        T: 'static,
        T: Source + Send + Sync,
    {
        self.component_read_guard::<Config>()
            .await
            .merge(source)
            .await
    }

    async fn set<T>(&self, key: &str, value: T) -> Result<()>
    where
        T: Into<Value> + Send,
    {
        self.component_read_guard::<Config>()
            .await
            .set(key, value)
            .await
    }
}
