use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        social::{SocialAuthManager, SocialAuthService},
        tokens::NewAPIAuthToken,
    },
    config::Config,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SocialAuthCallbackRequest {
    service: SocialAuthService,
    nonce: String,
    code: String,
    redirect_url: String,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct SocialAuthCallbackResponse {
    token: NewAPIAuthToken,
    new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
}

#[api_operation(tag = "Authentication", operation_id = "auth.social.callback")]
pub async fn auth_social_callback(
    data: Json<SocialAuthCallbackRequest>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
    stripe: Data<stripe::Client>,
) -> Result<Json<SocialAuthCallbackResponse>, APIError> {
    let client = SocialAuthManager::new(data.service.clone(), config.as_ref())
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
            stripe.as_ref(),
            data.code.clone(),
            data.nonce.clone(),
            data.redirect_url.clone(),
            config.as_ref(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("social auth token exchange", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(SocialAuthCallbackResponse { token, new_org_id }))
}
