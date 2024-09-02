use palform_tsid::{
    resources::{IDAdminPublicKey, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::SecretParts;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource,
    crypto::keys::CryptoKeyRepr,
    entity_managers::keys::UserKeyManager,
};

#[openapi(tag = "User keys", operation_id = "keys.get_backup")]
#[get("/users/me/orgs/<org_id>/keys/<key_id>/backup")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Option<String>>, (Status, Json<APIError>)> {
    if !UserKeyManager::verify_key_org_and_user(db.inner(), key_id, org_id, token.get_user_id())
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let key = UserKeyManager::get_key_with_id(db.inner(), key_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if let Some(private_key_bytes) = key.private_key_backup {
        let repr = CryptoKeyRepr::<SecretParts>::from_database_bytes(&private_key_bytes)
            .map_err(|e| APIError::report_internal_error("parse database private key", e))?;

        let repr_string = repr
            .to_pem_string()
            .map_err(|e| APIError::report_internal_error("create private key pem", e))?;

        Ok(Json(Some(repr_string)))
    } else {
        Ok(Json(None))
    }
}
