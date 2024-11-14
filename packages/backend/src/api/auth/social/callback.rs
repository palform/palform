use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        social::{SocialAuthManager, SocialAuthService},
        tokens::NewAPIAuthToken,
    },
    config::Config,
};

#[derive(Deserialize, JsonSchema)]
pub struct SocialAuthCallbackRequest {
    service: SocialAuthService,
    nonce: String,
    code: String,
    redirect_url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct SocialAuthCallbackResponse {
    token: NewAPIAuthToken,
    new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
}

#[openapi(tag = "Authentication", operation_id = "auth.social.callback")]
#[post("/auth/social/callback", data = "<data>")]
pub async fn handler(
    data: Json<SocialAuthCallbackRequest>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
    stripe: &State<stripe::Client>,
) -> Result<Json<SocialAuthCallbackResponse>, APIErrorWithStatus> {
    let client = SocialAuthManager::new(data.service.clone(), config)
        .await
        .map_err(|e| APIError::report_internal_error("discover client for callback", e))?;

    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let (token, new_org_id) = client
        .token_exchange(
            &txn,
            #[cfg(feature = "saas")]
            stripe,
            data.code.clone(),
            data.nonce.clone(),
            data.redirect_url.clone(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("social auth token exchange", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(SocialAuthCallbackResponse { token, new_org_id }))
}
