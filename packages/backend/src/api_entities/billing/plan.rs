use chrono::{DateTime, Utc};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;
use stripe::{Price, RecurringInterval};

use crate::billing::error::BillingError;

#[derive(Serialize, JsonSchema)]
pub struct APIBillingPlanPrice {
    pub stripe_price_id: String,
    pub amount: i64,
}

impl TryFrom<Price> for APIBillingPlanPrice {
    type Error = BillingError;
    fn try_from(value: Price) -> Result<Self, Self::Error> {
        Ok(Self {
            stripe_price_id: value.id.to_string(),
            amount: value
                .unit_amount
                .ok_or(BillingError::FieldNone("unit_amount".to_string()))?,
        })
    }
}

#[derive(Serialize, JsonSchema)]
pub struct APIBillingPlan {
    pub name: String,
    pub stripe_product_id: String,
    pub price_monthly: APIBillingPlanPrice,
    pub price_annually: APIBillingPlanPrice,
    pub currency: String,
    pub features: Vec<String>,
    pub highlight: bool,
}

#[derive(Serialize, JsonSchema)]
pub enum APIBillingSubscriptionFrequency {
    Monthly,
    Annually,
}
impl TryFrom<RecurringInterval> for APIBillingSubscriptionFrequency {
    type Error = BillingError;
    fn try_from(value: RecurringInterval) -> Result<Self, Self::Error> {
        Ok(match value {
            RecurringInterval::Month => APIBillingSubscriptionFrequency::Monthly,
            RecurringInterval::Year => APIBillingSubscriptionFrequency::Annually,
            _ => return Err(BillingError::UnsupportedFrequency),
        })
    }
}

#[derive(Serialize, JsonSchema)]
pub struct APIBillingSubscription {
    pub stripe_subscription_id: String,
    pub stripe_plan_product_id: String,
    pub stripe_plan_price_id: String,
    pub is_trial: bool,
    pub plan_name: String,
    pub plan_frequency: APIBillingSubscriptionFrequency,
    pub currency: String,
    pub price: APIBillingPlanPrice,
    pub period_end: DateTime<Utc>,
    pub canceling_at_end: bool,
}
