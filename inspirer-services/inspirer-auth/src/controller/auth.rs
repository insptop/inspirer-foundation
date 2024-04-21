use std::env::current_dir;

use axum_login::tower_sessions::Session;
use inspirer_framework::{
    extract::{Query, Request, State},
    preludes::*,
    routing::get,
    tower::ServiceExt,
    tower_http::services::{ServeDir, ServeFile},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::app::App;

#[derive(Debug, Deserialize)]
pub struct LoginParams {
    app_id: Uuid,
}

pub async fn login(
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

pub fn routes() -> Router<App> {
    let path = current_dir()
        .unwrap()
        .join("inspirer-services/inspirer-auth/auth-page/dist");

    assert!(path.exists());
    assert!(path.join("index.html").exists());

    Router::new()
        .route_service("/vite.svg", ServeFile::new(path.join("vite.svg")))
        .route("/login", get(login))
        .nest_service("/assets", ServeDir::new(path.join("assets")))
}
