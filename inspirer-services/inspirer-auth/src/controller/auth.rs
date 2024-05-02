use std::env::current_dir;

use axum_login::tower_sessions::Session;
use inspirer_framework::{
    extract::{Json, Query, Request, State},
    preludes::*,
    routing::get,
    tower::ServiceExt,
    tower_http::services::{ServeDir, ServeFile},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::App,
    auth::user::UserCredential,
    config::AppConfig,
    entity::users,
    service::{user::User, ServiceInterface},
};

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    app_id: Uuid,
}

pub async fn auth_page(
    State(app): State<AppContext<App>>,
    Query(params): Query<LoginParams>,
    session: Session,
    req: Request,
) -> Result<impl IntoResponse> {
    let path = current_dir()
        .unwrap()
        .join("inspirer-services/inspirer-auth/auth-page/dist");

    let app_id: Option<Uuid> = session.get::<Uuid>("app_id").await.map_err(Error::wrap)?;

    if let Some(app_id) = app_id {
        if params.app_id != app_id {
            return Err(Error::string("Invalid request"));
        }
    } else {
        session
            .insert("app_id", params.app_id)
            .await
            .map_err(Error::wrap)?;
    }

    let service = ServeFile::new(path.join("index.html"));
    service.oneshot(req).await.map_err(Error::wrap)
}

#[derive(Deserialize)]
pub struct LoginRequest {
    /// 登录凭据
    credential: UserCredential,
}

pub async fn login(
    State(app): State<AppContext<App>>,
    Json(payload): Json<LoginRequest>,
    session: Session,
) -> Resp<()> {
    let app_id = session
        .get::<Uuid>("app_id")
        .await
        .map_err(Error::wrap)?
        .ok_or(Error::string("Invalid request"))?;

    tracing::trace!("Received login request, app id = {app_id}");

    let user = app
        .service::<User>()
        .find_user_by_credential(payload.credential)
        .await?;

    // generate id token and profile data

    ok(())
}

pub fn routes(app: &AppContext<App>) -> Router<App> {
    let config = app
        .config
        .get::<AppConfig>("app")
        .expect("Missing app config");

    let path = current_dir().unwrap().join(config.default_auth_page);

    assert!(path.exists());
    assert!(path.join("index.html").exists());

    Router::new()
        .route_service("/vite.svg", ServeFile::new(path.join("vite.svg")))
        .route("/login", get(auth_page))
        .nest_service("/assets", ServeDir::new(path.join("assets")))
}
