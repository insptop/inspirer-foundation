//! Inspirer 应用部分的服务层依赖的必要功能组件集成的支持

use std::sync::Arc;

use crate::{Result, Error};

#[cfg(feature = "enable-axum")]
use axum::extract::{FromRequest, RequestParts};

use tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, RwLockMappedWriteGuard};
use type_map::concurrent::TypeMap;

pub mod config;
pub mod database;

#[async_trait]
pub trait ComponentConstructor {
    async fn constructor(&self, service: ComponentProvider) -> Result<()>;
}

#[async_trait]
impl<T> ComponentConstructor for Box<T> 
where T: ComponentConstructor + Send + Sync
{
    async fn constructor(&self, service: ComponentProvider) -> Result<()> {
        self.as_ref().constructor(service).await
    }
}

#[derive(Clone, Default)]
pub struct ComponentProvider {
    inner: Inner,
}

#[derive(Clone, Default)]
struct Inner {
    pub(crate) components: Arc<RwLock<TypeMap>>,
}

#[derive(Default)]
pub struct ComponentProviderBuilder {
    components: Vec<Box<dyn ComponentConstructor>>
}

impl ComponentProviderBuilder {
    pub fn provide<T: ComponentConstructor + 'static>(&mut self, cp: T) {
        self.components.push(Box::new(cp));
    }

    pub async fn build(&self) -> Result<ComponentProvider> {
        let service = ComponentProvider::default();

        for cp in self.components.iter() {
            cp.constructor(service.clone()).await?;
        }

        Ok(service)
    }
}

impl ComponentProvider {
    /// 获取组件，如果没有则 panic
    pub async fn component<T: 'static + Clone>(&self) -> T {
        self.try_get_component()
            .await
            .expect("Component is not found.")
    }

    pub async fn try_get_component<T: 'static + Clone>(&self) -> Option<T> {
        self.inner.components.read().await.get::<T>().cloned()
    }

    /// 获取带读锁的组件，用于只读场景
    pub async fn component_read_guard<T: 'static>(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard::map(self.inner.components.read().await, |inner| {
            inner.get().expect("Component is not found.")
        })
    }

    /// 获取带写锁的组件，用于读写场景
    pub async fn component_write_guard<T: 'static>(&self) -> RwLockMappedWriteGuard<'_, T> {
        RwLockWriteGuard::map(self.inner.components.write().await, |inner| {
            inner.get_mut().expect("Component is not found.")
        })
    }

    /// 注册组件
    pub async fn register_component<T>(&self, component: T)
    where
        T: 'static + Clone + Send + Sync,
    {
        let mut guard = self.inner.components.write().await;

        guard.insert(component);
        log::info!("Registered component <{}>", std::any::type_name::<T>());
    }
}

#[cfg(feature = "enable-axum")]
#[async_trait]
impl<B> FromRequest<B> for ComponentProvider
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        log::debug!("Extract Service extension.");
        let res = req
            .extensions()
            .ok_or_else(|| {
                log::error!("Extract service extension failed. Extension module is not found.");
                Error::ExtractServiceExtensionFailed
            })?
            .get::<Self>()
            .ok_or_else(|| {
                log::error!("Service extension is not found.");
                Error::ExtractServiceExtensionFailed
            })
            .map(|res| res.clone())?;

        Ok(res)
    }
}
