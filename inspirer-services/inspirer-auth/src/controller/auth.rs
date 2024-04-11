use std::time::Duration;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use chrono::Utc;
use inspirer_framework::{preludes::*, response::ErrorDetail};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::{
    app::App, entity::users, header::AppId, password::password_verify, token::AccessToken,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// 登录凭据
    credential: LoginCredential,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum LoginCredential {
    Username { username: String, password: String },
    Email { email: String, password: String },
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token_type: &'static str,
    access_token: String,
}

pub async fn login(
    TypedHeader(app_id): TypedHeader<AppId>,
    State(app): State<AppContext<App>>,
    Json(req): Json<LoginRequest>,
) -> Resp<LoginResponse> {
    tracing::debug!("Received login request, appId = {}", app_id.0);
    let mut finder = users::Entity::find();

    let password = match req.credential {
        LoginCredential::Username { username, password } => {
            tracing::debug!("login use username credential, username = {username}, password = {password}");
            finder = finder.filter(users::Column::Username.eq(username));
            password
        }
        LoginCredential::Email { email, password } => {
            finder = finder.filter(users::Column::Email.eq(email));
            password
        }
    };

    let user = finder.one(&app.database).await?.ok_or(Error::CustomError(
        StatusCode::NOT_FOUND,
        ErrorDetail::with_reason("User not exists"),
    ))?;

    if password_verify(password, user.password).is_err() {
        return Err(Error::Unauthorized(
            "User not exists or password error".into(),
        ));
    }

    let claims = AccessToken {
        aud: app_id.0,
        sub: user.uuid,
        scope: "openid profile email phone".into(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now() + Duration::from_secs(3600)).timestamp() as usize,
    };

    ok(LoginResponse {
        token_type: "Bearer",
        access_token: claims.token(),
    })
}
