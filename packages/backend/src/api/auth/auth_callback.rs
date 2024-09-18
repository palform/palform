use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    audit::AuditManager,
    auth::{oidc::OIDCManager, tokens::NewAPIAuthToken},
    rocket_util::from_org_id::FromOrgIdTrait,
};

#[derive(Deserialize, JsonSchema)]
pub struct AuthCallbackRequest {
    /// The authorization code provided in response from the OIDC provider
    auth_code: String,
    /// The nonce provided when starting the auth flow
    nonce: String,
    /// The redirect URL set by the client when starting the auth flow
    redirect_url: String,
}

#[derive(Serialize, JsonSchema)]
pub struct AuthCallbackResponse {
    token: NewAPIAuthToken,
}

/// Process authentication callback
///
/// Handle the callback from the OIDC server and generate an API key to be used for future requests
#[openapi(tag = "Authentication", operation_id = "auth.callback")]
#[post("/auth/<org_id>/callback", data = "<request>", rank = 2)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    request: Json<AuthCallbackRequest>,
    db: &State<DatabaseConnection>,
) -> Result<Json<AuthCallbackResponse>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let client = OIDCManager::get_client_for_org(&txn, org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get org OIDC client", e))?;

    let (token, user_id) = client
        .token_exchange(
            &txn,
            request.auth_code.clone(),
            request.nonce.clone(),
            request.redirect_url.clone(),
        )
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    AuditManager::new(org_id)
        .log_event_with_note(
            &txn,
            user_id,
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::AuthSession,
            Some(token.id.into_unknown()),
            Some("Signed in with OIDC".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(AuthCallbackResponse { token }))
}
