use std::sync::Arc;

use crate::Result;
use config::{Config as LocalRepository, Source, Value};
use serde::Deserialize;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
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
