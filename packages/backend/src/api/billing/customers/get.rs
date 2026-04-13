use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::customer::APIBillingCustomer, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingCustomersGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Customers", operation_id = "billing.customer.get")]
pub async fn billing_customers_get(
    path: Path<BillingCustomersGetPath>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIBillingCustomer>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let customer = manager
        .get_org_customer(db.as_ref(), path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get customer", e))?;

    Ok(Json(customer))
}
