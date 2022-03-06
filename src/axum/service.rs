use axum::extract::{FromRequest, RequestParts};
pub use axum::http::Extensions;

use crate::{service::*, Error, Result};

/// 应用内部服务层对象构造器
#[async_trait]
pub trait ServiceProvider: Sized {
    async fn provide(extensions: &Extensions) -> Result<Self>;
}

#[async_trait]
impl<B, S> FromRequest<B> for Service<S>
where
    B: Send,
    S: ServiceProvider
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        log::debug!("Extract Service<{}>.", std::any::type_name::<S>());
        let extensions = req
            .extensions()
            .ok_or_else(|| {
                log::error!("No extensions.");
                Error::ExtractServiceExtensionFailed
            })?;

        <S as ServiceProvider>::provide(extensions).await.map(Service)
    }
}
