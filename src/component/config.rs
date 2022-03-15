use crate::Result;
use config::Config as LocalRepository;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

pub use config::{Environment, File, FileSourceFile, FileSourceString, Source, ConfigError, Value};

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

impl Config {
    pub fn new<T: 'static + Source + Send + Sync>(source: T) -> Result<Self> {
        Ok(LocalRepository::default().with_merged(source)?.into())
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
