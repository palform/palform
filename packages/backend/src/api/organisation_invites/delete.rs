use palform_tsid::{resources::{IDOrganisation, IDOrganisationInvite}, tsid::PalformDatabaseID};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[openapi(
    tag = "Organisation Invites",
    operation_id = "organisation.invites.delete"
)]
#[delete("/users/me/orgs/<org_id>/invites/<invite_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    invite_id: PalformDatabaseID<IDOrganisationInvite>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
) -> Result<(), (Status, Json<APIError>)> {
    if !OrganisationInviteManager::verify_invite_org(db.inner(), invite_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    OrganisationInviteManager::delete(db.inner(), invite_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    Ok(())
}
