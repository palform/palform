use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::organisation_team::APIOrganisationTeam, auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Teams", operation_id = "organisation.teams.list")]
pub async fn organisation_teams_list(
    path: Path<OrganisationTeamsListPath>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeam>>, APIError> {
    let resp = OrganisationTeamsManager::list_org_teams(db.as_ref(), path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    Ok(Json(resp))
}
