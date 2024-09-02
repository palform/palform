use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalError};
use palform_tsid::{resources::{IDAdminUser, IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[openapi(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.delete"
)]
#[delete("/users/me/orgs/<org_id>/teams/<team_id>/members/<member_user_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    member_user_id: PalformDatabaseID<IDAdminUser>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    if !OrganisationTeamsManager::verify_is_member(db.inner(), team_id, member_user_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !OrganisationTeamsManager::verify_team_org(db.inner(), team_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    OrganisationTeamsManager::remove_from_team(db.inner(), team_id, member_user_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(())
}
