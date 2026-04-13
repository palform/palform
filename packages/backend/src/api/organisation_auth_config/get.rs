use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::organisation_auth_config::APIOrganisationAuthConfig,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_auth_config::OrganisationAuthConfigManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationAuthConfigGetPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Authentication Configuration", operation_id = "organisation.auth_config.get")]
pub async fn organisation_auth_config_get(
    _path: Path<OrganisationAuthConfigGetPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    m: FromOrgId<OrganisationAuthConfigManager>,
) -> Result<Json<Option<APIOrganisationAuthConfig>>, APIError> {
    let config = m.get(db.as_ref()).await.map_internal_error()?;
    Ok(Json(config))
}
