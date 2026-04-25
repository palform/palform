use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::invoice::APIBillingInvoice, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingInvoicesListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Invoices", operation_id = "billing.invoice.list")]
pub async fn billing_invoices_list(
    path: Path<BillingInvoicesListPath>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIBillingInvoice>>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let invoices = manager
        .list_org_invoices(db.as_ref(), path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("list org invoices", e))?;

    Ok(Json(invoices))
}
