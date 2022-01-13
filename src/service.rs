use std::sync::Arc;

use crate::Error;

#[cfg(feature = "enable-axum")]
use axum::extract::{FromRequest, RequestParts};

use tokio::sync::{RwLock, RwLockReadGuard};
use type_map::concurrent::TypeMap;

#[derive(Clone, Default)]
pub struct Service {
    inner: ServiceInner,
}

#[derive(Clone, Default)]
struct ServiceInner {
    pub(crate) components: Arc<RwLock<TypeMap>>,
}

impl Service {
    pub async fn component<T: 'static + Clone>(&self) -> T {
        self.try_get_component()
            .await
            .expect("Component is not found.")
    }

    pub async fn try_get_component<T: 'static + Clone>(&self) -> Option<T> {
        self.inner.components.read().await.get::<T>().cloned()
    }

    pub async fn component_guard<T: 'static>(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard::map(self.inner.components.read().await, |inner| {
            inner.get().expect("Component is not found.")
        })
    }

    pub async fn register_component<T>(&self, component: T)
    where
        T: 'static + Clone + Send + Sync,
    {
        let mut guard = self.inner.components.write().await;

        guard.insert(component);
    }
}

#[cfg(feature = "enable-axum")]
#[async_trait]
impl<B> FromRequest<B> for Service
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
