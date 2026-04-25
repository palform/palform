use actix_web::web::{Data, Json, Query};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::entity_managers::orgs::OrganisationManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub(crate) struct OrganisationsResolveSubdomainQuery {
    subdomain: String,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.resolve_subdomain")]
pub async fn organisations_resolve_subdomain(
    Query(query): Query<OrganisationsResolveSubdomainQuery>,
    db: Data<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDOrganisation>>, APIError> {
    let org_id = OrganisationManager::get_org_for_subdomain(db.as_ref(), query.subdomain)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;
    Ok(Json(org_id))
}
