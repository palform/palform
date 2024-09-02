use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::customer::APIBillingCustomer, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[openapi(tag = "Billing Customers", operation_id = "billing.customer.get")]
#[get("/users/me/orgs/<org_id>/billing/customer")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIBillingCustomer>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let customer = manager
        .get_org_customer(db.inner(), org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get customer", e))?;

    Ok(Json(customer))
}
