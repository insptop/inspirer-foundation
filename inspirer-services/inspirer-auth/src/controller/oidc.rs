use std::str::FromStr;

use inspirer_framework::{
    axum::response::IntoResponse,
    extract::{Path, Query, State},
    http::{header::LOCATION, HeaderValue},
    preludes::*,
    routing::{get, on, MethodFilter},
};
use openidconnect::{
    core::{
        CoreClaimName, CoreJwsSigningAlgorithm, CoreProviderMetadata, CoreResponseType,
        CoreSubjectIdentifierType,
    },
    AuthUrl, EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeySetUrl, ResponseTypes, Scope,
    TokenUrl, UserInfoUrl,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{app::App, auth::ocid::AuthenticationRequest, config::AppConfig, entity::apps};

pub async fn openid_configuration(
    Path((app_id,)): Path<(Uuid,)>,
    State(context): State<AppContext<App>>,
) -> Result<Json<impl Serialize>> {
    let app = apps::Entity::find()
        .filter(apps::Column::Uuid.eq(app_id))
        .one(&context.database)
        .await?
        .ok_or(Error::NotFound)?;

    let meta = CoreProviderMetadata::new(
        IssuerUrl::from_url(app.setting.base_setting.endpoint.join("/oidc").unwrap()),
        AuthUrl::from_url(app.setting.base_setting.endpoint.join("/oidc/auth")?),
        JsonWebKeySetUrl::from_url(
            app.setting
                .base_setting
                .endpoint
                .join("/oidc/.well-known/jwks.json")?,
        ),
        vec![ResponseTypes::new(vec![CoreResponseType::Code])],
        vec![CoreSubjectIdentifierType::Public],
        vec![CoreJwsSigningAlgorithm::EcdsaP256Sha256],
        EmptyAdditionalProviderMetadata {},
    )
    .set_token_endpoint(Some(TokenUrl::from_url(
        app.setting.base_setting.endpoint.join("/oidc/token")?,
    )))
    .set_userinfo_endpoint(Some(UserInfoUrl::from_url(
        app.setting.base_setting.endpoint.join("/oidc/userinfo")?,
    )))
    .set_scopes_supported(Some(vec![
        Scope::new("openid".to_string()),
        Scope::new("email".to_string()),
        Scope::new("profile".to_string()),
    ]))
    .set_claims_supported(Some(vec![
        CoreClaimName::new("sub".to_string()),
        CoreClaimName::new("aud".to_string()),
        CoreClaimName::new("email".to_string()),
        CoreClaimName::new("email_verified".to_string()),
        CoreClaimName::new("exp".to_string()),
        CoreClaimName::new("iat".to_string()),
        CoreClaimName::new("iss".to_string()),
        CoreClaimName::new("name".to_string()),
        CoreClaimName::new("given_name".to_string()),
        CoreClaimName::new("family_name".to_string()),
        CoreClaimName::new("picture".to_string()),
        CoreClaimName::new("locale".to_string()),
    ]))
    .set_request_parameter_supported(Some(false))
    .set_claims_parameter_supported(Some(false));

    Ok(Json(meta))
}

pub async fn auth(
    State(app): State<AppContext<App>>,
    Query(params): Query<AuthenticationRequest>,
) -> Result<impl IntoResponse> {
    let config = app.config.get::<AppConfig>("app")?;

    let mut location = config.app_endpoint.join("login")?;

    // TODO not finished

    let uuid = Uuid::from_str(&params.client_id)
        .map_err(|_| Error::BadRequest("Invalid client id".to_string()))?;

    location
        .query_pairs_mut()
        .append_pair("app_id", &uuid.to_string());

    Ok((
        StatusCode::FOUND,
        [(
            LOCATION,
            HeaderValue::try_from(location.to_string()).expect("URI isn't a valid header value"),
        )],
    ))
}

pub fn routes() -> Router<App> {
    Router::new()
        .route(
            "/app/:appid/oidc/.well-known/openid-configuration",
            get(openid_configuration),
        )
        .route(
            "/app/login",
            on(MethodFilter::GET.or(MethodFilter::POST), auth),
        )
}
