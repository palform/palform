use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminPublicKey, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{put, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::SecretParts;
use serde::Deserialize;

use crate::api::error::{APIError, APIInternalError};
use crate::audit::AuditManager;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::crypto::keys::CryptoKeyRepr;
use crate::entity_managers::keys::UserKeyManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(Deserialize, JsonSchema)]
pub struct RegisterBackupKeyRequest {
    /// PEM-encoded encrypted secret key
    key_data: Option<String>,
}

#[openapi(tag = "User keys", operation_id = "keys.register_backup")]
#[put("/users/me/orgs/<org_id>/keys/<key_id>/backup", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
    data: Json<RegisterBackupKeyRequest>,
    token: APITokenOrgViewer,
    audit: FromOrgId<AuditManager>,
    db: &State<DatabaseConnection>,
) -> Result<(), (Status, Json<APIError>)> {
    if !UserKeyManager::verify_key_org_and_user(db.inner(), key_id, org_id, token.get_user_id())
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if let Some(key_data) = data.key_data.clone() {
        let key_data = CryptoKeyRepr::<SecretParts>::from_pem_string(&key_data)
            .map_err(|e| APIError::BadRequest(e.to_string()))?;
        UserKeyManager::register_user_key_backup(db.inner(), key_id, key_data)
            .await
            .map_err(|e| APIError::report_internal_error("register user key backup", e))?;
    } else {
        UserKeyManager::delete_user_key_backup(db.inner(), key_id)
            .await
            .map_err(|e| e.to_internal_error())?;
    }

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::AdminPublicKey,
            Some(key_id.into_unknown()),
            Some("Registered encrypted backup key".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
