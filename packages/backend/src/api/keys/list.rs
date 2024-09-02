use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::State;
use rocket::{get, serde::json::Json};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::api_entities::key::APIUserKey;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::keys::UserKeyManager;

/// List user keys
///
/// Lists the public keys associated with the currently authenticated user's account in PEM-encoded
/// format.
#[openapi(tag = "User keys", operation_id = "keys.list")]
#[get("/users/me/orgs/<org>/keys", rank = 2)]
pub async fn handler(
    org: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIUserKey>>, APIErrorWithStatus> {
    let response = UserKeyManager::list_keys_for_user(db.inner(), token.get_user_id(), org)
        .await
        .map_internal_error()?;

    let mapped_response: Result<Vec<_>, _> = response
        .iter()
        .map(|e| APIUserKey::try_from(e.to_owned()))
        .collect();

    let mapped_response =
        mapped_response.map_err(|e| APIError::report_internal_error("parse user keys", e))?;

    Ok(Json(mapped_response))
}
