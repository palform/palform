use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{delete, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::billing::manager::CancelPlanRequestReason;
use crate::{auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager};

#[derive(Deserialize, JsonSchema)]
pub struct CancelPlanRequest {
    reason: CancelPlanRequestReason,
}

#[openapi(tag = "Billing Plans", operation_id = "billing.plan.cancel")]
#[delete(
    "/users/me/orgs/<org_id>/billing/plan/<stripe_subscription_id>",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    stripe_subscription_id: String,
    data: Json<CancelPlanRequest>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    manager
        .cancel_subscription(
            db.inner(),
            org_id,
            stripe_subscription_id,
            data.reason.clone().into(),
        )
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    Ok(())
}
