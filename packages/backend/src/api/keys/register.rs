use palform_tsid::resources::{IDAdminPublicKey, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use sequoia_openpgp::packet::key::PublicParts;
use serde::Deserialize;

use crate::api::error::{APIError, APIInternalError};
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::crypto::keys::CryptoKeyRepr;
use crate::entity_managers::keys::UserKeyManager;

#[derive(Deserialize, JsonSchema)]
pub struct RegisterKeyRequest {
    /// PEM-encoded public key
    key_data: String,
}

/// Register new public key
///
/// Registers a new public to the authenticated user's account. The key is stored in DER-encoded
/// binary in the database and can be retrieved using the GET /users/me/key endpoint.
#[openapi(tag = "User keys", operation_id = "keys.register")]
#[post("/users/me/orgs/<org>/keys", data = "<data>")]
pub async fn handler(
    token: APITokenOrgViewer,
    org: PalformDatabaseID<IDOrganisation>,
    db: &State<DatabaseConnection>,
    data: Json<RegisterKeyRequest>,
) -> Result<Json<PalformDatabaseID<IDAdminPublicKey>>, (Status, Json<APIError>)> {
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

    let new_key = UserKeyManager::register_key_for_user(&txn, token.get_user_id(), org, cert)
        .await
        .map_err(|e| APIError::report_internal_error("register key", e))?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(new_key.id))
}
