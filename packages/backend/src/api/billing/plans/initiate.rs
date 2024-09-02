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
pub struct InitiatePlanRequest {
    stripe_plan_price_id: String,
    success_url: String,
    trial: bool,
}

#[openapi(tag = "Billing Plans", operation_id = "billing.plan.initiate")]
#[post("/users/me/orgs/<org_id>/billing/plan.initiate", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<InitiatePlanRequest>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<Json<String>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let url = manager
        .create_checkout_session(
            db.inner(),
            org_id,
            data.stripe_plan_price_id.clone(),
            data.trial,
            data.success_url.clone(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("create checkout session", e))?;

    Ok(Json(url))
}
