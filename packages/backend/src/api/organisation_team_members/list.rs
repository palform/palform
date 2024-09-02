use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalError};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::organisation_team::APIOrganisationTeamMember,
    auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[openapi(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.list"
)]
#[get("/users/me/orgs/<org_id>/teams/<team_id>/members")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeamMember>>, APIErrorWithStatus> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    if !OrganisationTeamsManager::verify_team_org(db.inner(), team_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let resp = OrganisationTeamsManager::list_members_for_team(db.inner(), team_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(resp))
}
