use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::organisation_auth_team_mapping::APIOrganisationAuthTeamMapping,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Authentication Team Mappings",
    operation_id = "organisation.auth_config.mappings.list"
)]
#[get("/users/me/orgs/<_org_id>/auth_config/mappings")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<Json<Vec<APIOrganisationAuthTeamMapping>>, APIErrorWithStatus> {
    let mappings = m.list(db.inner()).await.map_internal_error()?;
    Ok(Json(mappings))
}
