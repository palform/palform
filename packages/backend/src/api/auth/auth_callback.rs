use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{oidc::OIDCManager, tokens::NewAPIAuthToken},
    config::Config,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub(crate) struct AuthCallbackRequest {
    /// The authorization code provided in response from the OIDC provider
    auth_code: String,
    /// The nonce provided when starting the auth flow
    nonce: String,
    /// The redirect URL set by the client when starting the auth flow
    redirect_url: String,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub(crate) struct AuthCallbackResponse {
    token: NewAPIAuthToken,
    is_new: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AuthCallbackPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

/// Process authentication callback
///
/// Handle the callback from the OIDC server and generate an API key to be used for future requests
#[api_operation(tag = "Authentication", operation_id = "auth.callback")]
pub async fn auth_callback(
    path: Path<AuthCallbackPath>,
    request: Json<AuthCallbackRequest>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
    audit_manager: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<Json<AuthCallbackResponse>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let client = OIDCManager::get_client_for_org(&txn, path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get org OIDC client", e))?;

    let (token, user_id, is_new) = client
        .token_exchange(
            &txn,
            request.auth_code.clone(),
            request.nonce.clone(),
            request.redirect_url.clone(),
            config.as_ref(),
        )
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    audit_manager
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
    Ok(Json(AuthCallbackResponse { token, is_new }))
}
