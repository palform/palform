use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::invoice::APIBillingInvoice, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[openapi(tag = "Billing Invoices", operation_id = "billing.invoice.list")]
#[get("/users/me/orgs/<org_id>/billing/invoices")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIBillingInvoice>>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let invoices = manager
        .list_org_invoices(db.inner(), org_id)
        .await
        .map_err(|e| APIError::report_internal_error("list org invoices", e))?;

    Ok(Json(invoices))
}
