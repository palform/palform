use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::organisation_auth_config::APIOrganisationAuthConfig,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_auth_config::OrganisationAuthConfigManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Authentication Configuration",
    operation_id = "organisation.auth_config.get"
)]
#[get("/users/me/orgs/<_org_id>/auth_config")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    m: FromOrgId<OrganisationAuthConfigManager>,
) -> Result<Json<Option<APIOrganisationAuthConfig>>, APIErrorWithStatus> {
    let config = m.get(db.inner()).await.map_internal_error()?;
    Ok(Json(config))
}
