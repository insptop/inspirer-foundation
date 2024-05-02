use axum_login::tower_sessions::{
    cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer, SessionStore,
};
use inspirer_framework::{command::CommandRegister, preludes::*};
use sea_orm::DbConn;
use tower_sessions_redis_store::{
    fred::{clients::RedisPool, interfaces::ClientLike, types::RedisConfig},
    RedisStore,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    command,
    config::{AppConfig, SessionDriverConfig},
    controller,
};

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

    async fn routes(app: AppContext<Self>) -> Result<Router<Self>> {
        let app_config = app.config.get::<AppConfig>("app")?;

        let router = Router::new()
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .merge({
                let router = controller::auth::routes(&app);
                match &app_config.session.driver {
                    SessionDriverConfig::Memory => router.layer(build_session_manage_layer(
                        &app_config,
                        MemoryStore::default(),
                    )),
                    SessionDriverConfig::Redis {
                        database_url,
                        pool_size,
                    } => {
                        let pool = RedisPool::new(
                            RedisConfig::from_url(&database_url).map_err(Error::wrap)?,
                            None,
                            None,
                            None,
                            *pool_size,
                        )
                        .map_err(Error::wrap)?;
                        let _ = pool.connect();
                        pool.wait_for_connect().await.map_err(Error::wrap)?;
                        let session_store = RedisStore::new(pool);

                        router.layer(build_session_manage_layer(&app_config, session_store))
                    }
                }
            })
            .merge(controller::api::routes())
            .merge(controller::oidc::routes());

        Ok(router)
    }

    fn commands(register: &mut CommandRegister<Self>) {
        register.register::<command::init::InitData>("app:init");
        register.register::<command::list::List>("app:list");
    }
}

fn build_session_manage_layer<T>(config: &AppConfig, store: T) -> SessionManagerLayer<T>
where
    T: SessionStore,
{
    SessionManagerLayer::new(store)
        .with_name(
            config
                .session
                .session_name
                .clone()
                .unwrap_or("auth_session".into()),
        )
        .with_expiry(
            config
                .session
                .session_expiry
                .clone()
                .unwrap_or(Expiry::OnInactivity(Duration::hours(24))),
        )
        .with_secure(config.session.with_secure.unwrap_or(false))
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
