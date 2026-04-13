use actix_web::{
    web::{Data, Json},
};
use apistos::api_operation;
use palform_client_common::errors::error::APIError;

use crate::{
    api_entities::billing::plan::{APIBillingCurrencyResponse, APIBillingPlan},
    billing::{client_currency::ClientCurrency, manager::BillingManager},
};

#[api_operation(tag = "Billing Plans", operation_id = "billing.plan.list")]
pub async fn billing_plans_list(
    currency: ClientCurrency,
    stripe: Data<stripe::Client>,
) -> Result<Json<APIBillingCurrencyResponse<Vec<APIBillingPlan>>>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let plans = manager
        .list_plans(currency.clone().into())
        .await
        .map_err(|e| APIError::report_internal_error("list plans", e))?;

    Ok(Json(APIBillingCurrencyResponse {
        currency: currency.to_string(),
        data: plans,
    }))
}
