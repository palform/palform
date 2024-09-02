use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalError};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::organisation_team::APIOrganisationTeam, auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[openapi(tag = "Organisation Teams", operation_id = "organisation.teams.list")]
#[get("/users/me/orgs/<org_id>/teams")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeam>>, APIErrorWithStatus> {
    let resp = OrganisationTeamsManager::list_org_teams(db.inner(), org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    Ok(Json(resp))
}
