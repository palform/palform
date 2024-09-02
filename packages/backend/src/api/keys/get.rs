use palform_tsid::{resources::{IDAdminPublicKey, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::key::APIUserKey,
    auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource,
    entity_managers::keys::UserKeyManager,
};

#[openapi(tag = "User keys", operation_id = "keys.get")]
#[get("/users/me/orgs/<org_id>/keys/<key_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIUserKey>, (Status, Json<APIError>)> {
    let key = UserKeyManager::get_key_with_id(db.inner(), key_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if key.organisation_id != org_id {
        return Err(APIError::NotFound.into());
    }
    if key.user_id != token.get_user_id() {
        return Err(APIError::NotAllowed.into());
    }

    Ok(Json(key.try_into().map_err(|e| {
        APIError::report_internal_error("convert key", e)
    })?))
}
