use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::api::error::APIError;
use crate::api_entities::org::APIOrganisation;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::entity_managers::orgs::OrganisationManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.get")]
pub async fn organisations_get(
    path: Path<OrganisationsGetPath>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIOrganisation>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadOnly),
        )
        .await
        .expect("txn");

    let org = OrganisationManager::get_org_by_id(&txn, path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("find organisation", e))?
        .ok_or(APIError::NotFound)?;

    Ok(Json(org))
}
