use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDAdminPublicKey, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::key::APIUserKey,
    auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource,
    entity_managers::keys::UserKeyManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    key_id: PalformDatabaseID<IDAdminPublicKey>,
}

#[api_operation(tag = "User keys", operation_id = "keys.get")]
pub async fn keys_get(
    path: Path<KeysGetPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIUserKey>, APIError> {
    let key = UserKeyManager::get_key_with_id(db.as_ref(), path.key_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if key.organisation_id != path.org_id {
        return Err(APIError::NotFound.into());
    }
    if key.user_id != token.get_user_id() {
        return Err(APIError::NotAllowed.into());
    }

    Ok(Json(key.try_into().map_err(|e| {
        APIError::report_internal_error("convert key", e)
    })?))
}
