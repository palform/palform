use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager};

#[derive(Deserialize, JsonSchema)]
pub struct UpdatePaymentMethodRequest {
    pub redirect_url: String,
}

#[openapi(
    tag = "Billing Customers",
    operation_id = "billing.customer.update_payment_method"
)]
#[post(
    "/users/me/orgs/<org_id>/billing/customer/payment_method_update_link",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<UpdatePaymentMethodRequest>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<Json<String>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let link = manager
        .update_payment_method_link(db.inner(), org_id, data.redirect_url.clone())
        .await
        .map_err(|e| {
            APIError::report_internal_error("create link to update customer payment method", e)
        })?;

    Ok(Json(link))
}
