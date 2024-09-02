use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::APITokenOrgViewer, auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_members::OrganisationMembersManager,
};

#[openapi(
    tag = "Organisation Members",
    operation_id = "organisation.members.am_i_admin"
)]
#[get("/users/me/orgs/<org_id>/am-i-admin")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<bool>, APIErrorWithStatus> {
    let resp = OrganisationMembersManager::get_is_admin(db.inner(), org_id, token.get_user_id())
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    Ok(Json(resp))
}
