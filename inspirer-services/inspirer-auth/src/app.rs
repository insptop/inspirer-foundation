use axum::{
    routing::{get, post},
    Router,
};
use inspirer_framework::{command::CommandRegister, preludes::*};
use sea_orm::DbConn;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

    fn routes() -> axum::Router<AppContext<Self>> {
        Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge(controller::api::routes())
            .merge(controller::oidc::routes())
    }

    fn commands(register: &mut CommandRegister<Self>) {
        register.register::<command::init::InitData>("app:init");
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(crate::controller::api::login),
    components(schemas(
        crate::controller::api::LoginRequest,
        crate::controller::api::LoginCredential,
        crate::controller::api::LoginResponse
    ))
)]
pub struct ApiDoc;
