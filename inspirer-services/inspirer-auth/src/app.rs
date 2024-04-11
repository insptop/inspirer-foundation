use axum::{routing::{get, post}, Router};
use inspirer_framework::{command::CommandRegister, preludes::*};
use sea_orm::DbConn;

use crate::{command, controller};

#[derive(Clone)]
pub struct App {
    pub database: DbConn,
}

#[async_trait::async_trait]
impl AppTrait for App {
    fn app_name() -> &'static str {
        "inspirer_auth"
    }

    async fn init(booter: Booter) -> Result<Self> {
        Ok(App {
            database: booter.component().await?,
        })
    }

    async fn routes() -> axum::Router<AppContext<Self>> {
        Router::new()
            .route("/test", get(controller::test))
            .route("/test-err", get(controller::test_err))
            .route("/login", post(controller::auth::login))
    }

    fn commands(register: &mut CommandRegister<Self>) {
        register.register::<command::init::InitData>("app:init");
    }
}
