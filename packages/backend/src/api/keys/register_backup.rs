use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminPublicKey, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::SecretParts;
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api::error::{APIError, APIInternalError};
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::crypto::keys::CryptoKeyRepr;
use crate::entity_managers::keys::UserKeyManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct RegisterBackupKeyRequest {
    /// PEM-encoded encrypted secret key
    key_data: Option<String>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysRegisterBackup {
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
}

#[api_operation(tag = "User keys", operation_id = "keys.register_backup")]
pub async fn keys_register_backup(
    path: Path<KeysRegisterBackup>,
    data: Json<RegisterBackupKeyRequest>,
    token: APITokenOrgViewer,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    if !UserKeyManager::verify_key_org_and_user(
        db.as_ref(),
        path.key_id,
        path.org_id,
        token.get_user_id(),
    )
    .await
    .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if let Some(key_data) = data.key_data.clone() {
        let key_data = CryptoKeyRepr::<SecretParts>::from_pem_string(&key_data)
            .map_err(|e| APIError::BadRequest(e.to_string()))?;
        UserKeyManager::register_user_key_backup(db.as_ref(), path.key_id, key_data)
            .await
            .map_err(|e| APIError::report_internal_error("register user key backup", e))?;
    } else {
        UserKeyManager::delete_user_key_backup(db.as_ref(), path.key_id)
            .await
            .map_err(|e| e.to_internal_error())?;
    }

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::AdminPublicKey,
            Some(path.key_id.into_unknown()),
            Some("Registered encrypted backup key".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
