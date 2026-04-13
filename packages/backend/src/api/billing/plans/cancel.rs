use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::billing::manager::CancelPlanRequestReason;
use crate::{auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct CancelPlanRequest {
    reason: CancelPlanRequestReason,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingPlansCancelPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    stripe_subscription_id: String,
}

#[api_operation(tag = "Billing Plans", operation_id = "billing.plan.cancel")]
pub async fn billing_plans_cancel(
    path: Path<BillingPlansCancelPath>,
    data: Json<CancelPlanRequest>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    manager
        .cancel_subscription(
            db.as_ref(),
            path.org_id,
            path.stripe_subscription_id.clone(),
            data.reason.clone(),
        )
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    Ok(())
}
