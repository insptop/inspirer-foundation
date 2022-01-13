use crate::{service::Service, Result};

pub mod config;
pub mod database;

#[async_trait]
pub trait ComponentConstructor {
    async fn constructor(&self, service: Service) -> Result<()>;
}

#[async_trait]
impl<T> ComponentConstructor for Box<T> 
where T: ComponentConstructor + Send + Sync
{
    async fn constructor(&self, service: Service) -> Result<()> {
        self.as_ref().constructor(service).await
    }
}