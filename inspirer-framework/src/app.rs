use std::{ops::Deref, sync::Arc};

use tokio::runtime::Runtime;

use crate::{command::CommandRegister, component::ComponentProvider, config::Config, Result};

pub struct Booter {
    config: Config,
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

    pub fn config(&self) -> &Config {
        &self.config
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

/// Trait for define an application
#[async_trait::async_trait]
pub trait AppTrait: Sized + Clone + Send + Sync {
    fn app_name() -> &'static str;

    /// Init application and return application instance
    ///
    /// You can create app instance, register some components you need
    /// in application.
    ///
    /// For example:
    ///
    /// ```rust
    /// #[derive(Clone)]
    /// struct App {
    ///     pub database: DbConn,
    /// }
    ///
    /// async fn init(booter: Booter) -> Result<Self> {
    ///     Ok(App {
    ///         database: booter.component().await?,
    ///     })
    /// }
    /// ```
    async fn init(booter: Booter) -> Result<Self>;

    /// Register application routes
    fn routes() -> axum::Router<AppContext<Self>>;

    /// Register application cli commands
    ///
    /// ```rust
    /// // Must import `clap`, we need to use the lib to parse command
    /// use clap::Parse;
    /// use inspirer_framework::preludes::*;
    /// use sea_orm::DbConn;
    /// use axum::Router;
    ///
    /// #[derive(Clone)]
    /// struct App;
    ///
    /// // Define a command, and you can see `clap` document to know
    /// // how to define a command with that.
    /// #[derive(Debug, Parse)]
    /// struct TestCommand;
    ///
    /// #[async_trait::async_trait]
    /// impl AppCommand for TestCommand {
    ///     async fn execute(&self, context: AppContext<App>) -> Result<()> {
    ///         println!("hello");
    ///
    ///         Ok(())
    ///     }
    /// }
    ///
    /// #[async_trait::async_trait]
    /// impl AppTrait for App {
    ///     async fn init(booter: Booter) -> Result<Self> {
    ///         Ok(App)
    ///     }
    ///
    ///     fn routes() -> axum::Router<AppContext<Self>> {
    ///         Router::new()
    ///     }
    ///
    ///     fn commands(register: &mut CommandRegister<Self>) {
    ///         register.register::<TestCommand>("test");
    ///     }
    /// }
    /// ```
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
