use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminPublicKey, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use sequoia_openpgp::packet::key::PublicParts;
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
pub struct RegisterKeyRequest {
    /// PEM-encoded public key
    key_data: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysRegisterPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

/// Register new public key
///
/// Registers a new public to the authenticated user's account. The key is stored in DER-encoded
/// binary in the database and can be retrieved using the GET /users/me/key endpoint.
#[api_operation(tag = "User keys", operation_id = "keys.register")]
pub async fn keys_register(
    path: Path<KeysRegisterPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    data: Json<RegisterKeyRequest>,
) -> Result<Json<PalformDatabaseID<IDAdminPublicKey>>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let cert = CryptoKeyRepr::<PublicParts>::from_pem_string(data.key_data.as_str())
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    if UserKeyManager::check_fingerprint_exists(&txn, cert.fingerprint())
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::BadRequest("Certificate already exists".to_string()).into());
    }

    let new_key =
        UserKeyManager::register_key_for_user(&txn, token.get_user_id(), path.org_id, cert)
            .await
            .map_err(|e| APIError::report_internal_error("register key", e))?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::AdminPublicKey,
            Some(new_key.id.into_unknown()),
            Some("Registered new key for user".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(new_key.id))
}
