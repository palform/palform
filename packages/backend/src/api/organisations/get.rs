use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::State;
use rocket::{get, serde::json::Json};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::api::error::APIError;
use crate::api_entities::org::APIOrganisation;
use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::entity_managers::orgs::OrganisationManager;

#[openapi(tag = "Organisations", operation_id = "orgs.get")]
#[get("/users/me/orgs/<org_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIOrganisation>, (Status, Json<APIError>)> {
    let txn = db
        .inner()
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadOnly),
        )
        .await
        .expect("txn");

    let org = OrganisationManager::get_org_by_id(&txn, org_id)
        .await
        .map_err(|e| APIError::report_internal_error("find organisation", e))?
        .ok_or(APIError::NotFound)?;

    Ok(Json(org))
}
