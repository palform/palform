use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::entity_managers::orgs::OrganisationManager;

#[openapi(tag = "Organisations", operation_id = "orgs.resolve_subdomain")]
#[get("/orgs/for-subdomain?<subdomain>")]
pub async fn handler(
    subdomain: String,
    db: &State<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDOrganisation>>, APIErrorWithStatus> {
    let org_id = OrganisationManager::get_org_for_subdomain(db.inner(), subdomain)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;
    Ok(Json(org_id))
}
