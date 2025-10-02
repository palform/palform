use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDAdminPublicKey, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::AuditManager,
    auth::{
        rbac::requests::{APITokenOrgAdmin, APITokenOrgViewer},
        tokens::APIAuthTokenSource,
    },
    entity_managers::{billing_entitlement_proxy::BillingEntitlementManager, keys::UserKeyManager},
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "User keys", operation_id = "keys.delete")]
#[delete("/users/me/orgs/<org_id>/keys/<key_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
    token: APITokenOrgViewer,
    admin_token: Option<APITokenOrgAdmin>,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let key = UserKeyManager::get_key_with_id(&txn, key_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if key.organisation_id != org_id {
        return Err(APIError::NotFound.into());
    }

    if key.user_id != token.get_user_id() {
        if admin_token.is_some() {
            billing
                .check_entitlement(&txn, APIEntitlementRequest::CryptoDetails)
                .await?;
        } else {
            return Err(APIError::NotAllowed.into());
        }
    }

    UserKeyManager::delete_key_with_id(&txn, key_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::AdminPublicKey,
            Some(key_id.into_unknown()),
            Some("User deleted own key".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
