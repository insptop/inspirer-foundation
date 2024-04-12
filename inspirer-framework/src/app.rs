use std::{ops::Deref, sync::Arc};

use tokio::runtime::Runtime;

use crate::{command::CommandRegister, component::ComponentProvider, config::Config, Result};

pub struct Booter {
    pub config: Config,
}

impl Booter {
    pub fn new(config: Config) -> Self {
        Booter { config }
    }

    pub async fn component<T>(&self) -> Result<T>
    where
        T: ComponentProvider,
    {
        Ok(T::create(self.config.get(T::config_key())?)
            .await
            .map_err(|err| err.into())?)
    }
}

#[derive(Clone)]
pub struct AppContext<T> {
    pub app: T,
    pub config: Arc<Config>,
}

impl<T> AppContext<T> {
    pub fn new(app: T, config: Config) -> Self {
        AppContext {
            app,
            config: Arc::new(config),
        }
    }
}

impl<T> Deref for AppContext<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.app
    }
}

#[async_trait::async_trait]
pub trait AppTrait: Sized + Clone + Send + Sync {
    fn app_name() -> &'static str;

    /// Init application and return application instance
    async fn init(booter: Booter) -> Result<Self>;

    /// Register application routes
    fn routes() -> axum::Router<AppContext<Self>>;

    fn commands(_register: &mut CommandRegister<Self>) {}
}

pub(crate) async fn create_app<T>(booter: Booter) -> Result<AppContext<T>>
where
    T: AppTrait + 'static,
{
    let config = booter.config.clone();
    let app = T::init(booter).await?;
    Ok(AppContext::new(app, config))
}
