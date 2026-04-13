use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::organisation_team::APIOrganisationTeam, auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamsGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Organisation Teams", operation_id = "organisation.teams.get")]
pub async fn organisation_teams_get(
    path: Path<OrganisationTeamsGetPath>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIOrganisationTeam>, APIError> {
    let team = OrganisationTeamsManager::get_by_id(db.as_ref(), path.team_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if team.organisation_id != path.org_id {
        return Err(APIError::NotFound.into());
    }

    Ok(Json(team))
}
