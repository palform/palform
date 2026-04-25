use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct UpdatePaymentMethodRequest {
    pub redirect_url: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingCustomersUpdatePaymentMethodPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Customers", operation_id = "billing.customer.update_payment_method")]
pub async fn billing_customers_update_payment_method(
    path: Path<BillingCustomersUpdatePaymentMethodPath>,
    data: Json<UpdatePaymentMethodRequest>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<Json<String>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let link = manager
        .update_payment_method_link(db.as_ref(), path.org_id, data.redirect_url.clone())
        .await
        .map_err(|e| {
            APIError::report_internal_error("create link to update customer payment method", e)
        })?;

    Ok(Json(link))
}
