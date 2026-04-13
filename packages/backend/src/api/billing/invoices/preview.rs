use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::invoice::APIBillingUpcomingInvoice,
    auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingInvoicesPreviewQuery {
    stripe_subscription_id: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingInvoicesPreviewPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Invoices", operation_id = "billing.invoice.preview")]
pub async fn billing_invoices_preview(
    path: Path<BillingInvoicesPreviewPath>,
    Query(query): Query<BillingInvoicesPreviewQuery>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    stripe: Data<stripe::Client>,
) -> Result<Json<APIBillingUpcomingInvoice>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let resp = manager
        .preview_invoice(db.as_ref(), path.org_id, query.stripe_subscription_id)
        .await
        .map_err(|e| APIError::report_internal_error("preview next invoice", e))?;

    Ok(Json(resp))
}
