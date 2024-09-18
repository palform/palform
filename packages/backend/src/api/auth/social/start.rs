use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::social::{SocialAuthManager, SocialAuthService},
    config::Config,
};

#[derive(JsonSchema, Deserialize)]
pub struct StartSocialAuthRequest {
    service: SocialAuthService,
    redirect_url: String,
}

#[derive(JsonSchema, Serialize)]
pub struct StartSocialAuthResponse {
    url: String,
    state: String,
    nonce: String,
}

#[openapi(tag = "Authentication", operation_id = "auth.social.start")]
#[post("/auth/social/start", data = "<data>")]
pub async fn handler(
    data: Json<StartSocialAuthRequest>,
    config: &State<Config>,
) -> Result<Json<StartSocialAuthResponse>, APIErrorWithStatus> {
    let client = SocialAuthManager::new(data.service.clone(), config)
        .await
        .map_err(|e| APIError::report_internal_error("start social auth client", e))?;

    let (auth_url, state, nonce) = client
        .authorization_url(data.redirect_url.clone())
        .map_err(|e| APIError::report_internal_error("create auth url for social auth", e))?;

    Ok(Json(StartSocialAuthResponse {
        url: auth_url.to_string(),
        state: state.secret().to_owned(),
        nonce: nonce.secret().to_owned(),
    }))
}
