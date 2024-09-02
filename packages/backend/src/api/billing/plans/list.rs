use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;

use crate::{
    api_entities::billing::plan::APIBillingPlan,
    billing::{client_currency::ClientCurrency, manager::BillingManager},
};

#[openapi(tag = "Billing Plans", operation_id = "billing.plan.list")]
#[get("/billing/plans")]
pub async fn handler(
    currency: ClientCurrency,
    stripe: &State<stripe::Client>,
) -> Result<Json<Vec<APIBillingPlan>>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let plans = manager
        .list_plans(currency.into())
        .await
        .map_err(|e| APIError::report_internal_error("list plans", e))?;

    Ok(Json(plans))
}
