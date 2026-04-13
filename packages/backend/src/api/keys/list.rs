use crate::api::error::{APIError, APIInternalError};
use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::api_entities::key::APIUserKey;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::keys::UserKeyManager;

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct KeysListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

/// List user keys
///
/// Lists the public keys associated with the currently authenticated user's account in PEM-encoded
/// format.
#[api_operation(tag = "User keys", operation_id = "keys.list")]
pub async fn keys_list(
    path: Path<KeysListPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIUserKey>>, APIError> {
    let response =
        UserKeyManager::list_keys_for_user(db.as_ref(), token.get_user_id(), path.org_id)
            .await
            .map_err(|e| e.to_internal_error())?;

    let mapped_response: Result<Vec<_>, _> = response
        .iter()
        .map(|e| APIUserKey::try_from(e.to_owned()))
        .collect();

    let mapped_response =
        mapped_response.map_err(|e| APIError::report_internal_error("parse user keys", e))?;

    Ok(Json(mapped_response))
}
