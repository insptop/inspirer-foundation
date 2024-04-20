use std::time::Duration;

use axum_extra::TypedHeader;
use chrono::Utc;
use inspirer_framework::{extract::State, preludes::*, response::ErrorDetail, routing::post};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    app::App, entity::users, header::AppId, password::password_verify, token::AccessToken,
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    /// 登录凭据
    credential: LoginCredential,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum LoginCredential {
    /// 使用用户名作为登录凭据
    Username {
        /// 用户名
        username: String,
        /// 密码
        password: String,
    },
    /// 使用邮箱作为登录凭据
    Email {
        /// 邮箱
        email: String,
        /// 密码
        password: String,
    },
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    /// Token 类型
    token_type: &'static str,
    /// Access Token
    access_token: String,
}

/// 登录接口
#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 200, description = "Success", body = LoginResponse)
    ),
    request_body = LoginRequest,
    params(
        ("x-auth-app-id", Header, description = "待认证的App ID"),
    )
)]
pub async fn login(
    TypedHeader(app_id): TypedHeader<AppId>,
    State(app): State<AppContext<App>>,
    Json(req): Json<LoginRequest>,
) -> Resp<LoginResponse> {
    tracing::debug!("Received login request, appId = {}", app_id.0);
    let mut finder = users::Entity::find();

    let password = match req.credential {
        LoginCredential::Username { username, password } => {
            tracing::debug!(
                username = username,
                password = password,
                "login use username credential"
            );
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

pub fn routes() -> Router<App> {
    Router::new().route("/api/login", post(login))
}
