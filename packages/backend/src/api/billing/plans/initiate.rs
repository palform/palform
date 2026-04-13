use actix_web::{
    web::{Data, Json, Path},
};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{auth::rbac::requests::APITokenOrgAdmin, billing::manager::BillingManager};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct InitiatePlanRequest {
    stripe_plan_price_id: String,
    success_url: String,
    trial: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingPlansInitiatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Plans", operation_id = "billing.plan.initiate")]
pub async fn billing_plans_initiate(
    data: Json<InitiatePlanRequest>,
    path: Path<BillingPlansInitiatePath>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<Json<String>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let url = manager
        .create_checkout_session(
            db.as_ref(),
            path.org_id,
            data.stripe_plan_price_id.clone(),
            data.trial,
            data.success_url.clone(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("create checkout session", e))?;

    Ok(Json(url))
}
