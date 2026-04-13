use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::social::{SocialAuthManager, SocialAuthService},
    config::Config,
};

#[derive(JsonSchema, Deserialize, ApiComponent)]
pub struct StartSocialAuthRequest {
    service: SocialAuthService,
    redirect_url: String,
}

#[derive(JsonSchema, Serialize, ApiComponent)]
pub struct StartSocialAuthResponse {
    url: String,
    state: String,
    nonce: String,
}

#[api_operation(tag = "Authentication", operation_id = "auth.social.start")]
pub async fn auth_social_start(
    data: Json<StartSocialAuthRequest>,
    config: Data<Config>,
) -> Result<Json<StartSocialAuthResponse>, APIError> {
    let client = SocialAuthManager::new(data.service.clone(), config.as_ref())
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
