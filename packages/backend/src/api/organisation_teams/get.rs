use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalError};
use palform_tsid::{resources::{IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::organisation_team::APIOrganisationTeam, auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[openapi(tag = "Organisation Teams", operation_id = "organisation.teams.get")]
#[get("/users/me/orgs/<org_id>/teams/<team_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIOrganisationTeam>, APIErrorWithStatus> {
    let team = OrganisationTeamsManager::get_by_id(db.inner(), team_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if team.organisation_id != org_id {
        return Err(APIError::NotFound.into());
    }

    Ok(Json(team))
}
