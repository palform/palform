use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::invoice::APIBillingUpcomingInvoice,
    auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager,
};

#[openapi(tag = "Billing Invoices", operation_id = "billing.invoice.preview")]
#[get("/users/me/orgs/<org_id>/billing/invoices/next?<stripe_subscription_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    stripe_subscription_id: String,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    stripe: &State<stripe::Client>,
) -> Result<Json<APIBillingUpcomingInvoice>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let resp = manager
        .preview_invoice(db.inner(), org_id, stripe_subscription_id)
        .await
        .map_err(|e| APIError::report_internal_error("preview next invoice", e))?;

    Ok(Json(resp))
}
