use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::organisation_auth_team_mapping::APIOrganisationAuthTeamMapping,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationAuthTeamMappingsListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Authentication Team Mappings", operation_id = "organisation.auth_config.mappings.list")]
pub async fn organisation_auth_team_mappings_list(
    _path: Path<OrganisationAuthTeamMappingsListPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<Json<Vec<APIOrganisationAuthTeamMapping>>, APIError> {
    let mappings = m.list(db.as_ref()).await.map_internal_error()?;
    Ok(Json(mappings))
}
