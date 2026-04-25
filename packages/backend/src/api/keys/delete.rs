use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDAdminPublicKey, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::{APIError, APIInternalError},
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        rbac::requests::{APITokenOrgAdmin, APITokenOrgViewer},
        tokens::APIAuthTokenSource,
    },
    entity_managers::{billing_entitlement_proxy::BillingEntitlementManager, keys::UserKeyManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
}

#[api_operation(tag = "User keys", operation_id = "keys.delete")]
pub async fn keys_delete(
    path: Path<KeysDeletePath>,
    token: APITokenOrgViewer,
    admin_token: Option<APITokenOrgAdmin>,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let key = UserKeyManager::get_key_with_id(&txn, path.key_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if key.organisation_id != path.org_id {
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

    UserKeyManager::delete_key_with_id(&txn, path.key_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::AdminPublicKey,
            Some(path.key_id.into_unknown()),
            Some("User deleted own key".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
