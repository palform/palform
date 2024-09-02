use chrono::{DateTime, Utc};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;
use stripe::{Invoice, InvoiceStatus, Object};

use crate::billing::{error::BillingError, util::stripe_timestamp_to_chrono};

#[derive(Serialize, JsonSchema)]
pub struct APIBillingInvoice {
    pub id: String,
    pub amount: i64,
    pub currency: String,
    pub status: APIBillingInvoiceStatus,
    pub created: DateTime<Utc>,
    pub url: Option<String>,
}

#[derive(Serialize, JsonSchema)]
pub enum APIBillingInvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl From<InvoiceStatus> for APIBillingInvoiceStatus {
    fn from(value: InvoiceStatus) -> Self {
        match value {
            InvoiceStatus::Draft => Self::Draft,
            InvoiceStatus::Open => Self::Open,
            InvoiceStatus::Paid => Self::Paid,
            InvoiceStatus::Uncollectible => Self::Uncollectible,
            InvoiceStatus::Void => Self::Void,
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct APIBillingUpcomingInvoice {
    pub invoice_date: DateTime<Utc>,
    pub amount_due: i64,
    pub total_amount: i64,
    pub tax_amount: i64,
    pub starting_balance: i64,
    pub ending_balance: i64,
    pub currency: String,
    pub lines: Vec<APIBillingUpcomingInvoiceLine>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub next_payment_attempt: DateTime<Utc>,
    pub promotions: Vec<APIBillingUpcomingInvoicePromotion>,
}

#[derive(Serialize, JsonSchema)]
pub struct APIBillingUpcomingInvoiceLine {
    pub stripe_price_id: String,
    pub name: String,
    pub quantity: u64,
    pub unit_price: i64,
    pub unit_price_per: i64,
    pub total_price: i64,
    pub proration: bool,
    pub stripe_description: String,
}

#[derive(Serialize, JsonSchema)]
pub struct APIBillingUpcomingInvoicePromotion {
    pub name: String,
    pub percent_off: Option<f64>,
    pub amount_off: Option<i64>,
}

impl TryFrom<Invoice> for APIBillingUpcomingInvoice {
    type Error = BillingError;
    fn try_from(value: Invoice) -> Result<Self, Self::Error> {
        let mut lines = Vec::<APIBillingUpcomingInvoiceLine>::new();
        for line in value
            .lines
            .ok_or(BillingError::FieldNone("invoice.lines".to_string()))?
            .data
        {
            let price = line.price.ok_or(BillingError::FieldNone(
                "invoice.lines.data.price".to_string(),
            ))?;
            let product = price.product.clone();
            let product = product.ok_or(BillingError::FieldNone(
                "invoice.lines.data.price.product".to_string(),
            ))?;
            let product = product.as_object().ok_or(BillingError::FieldNone(
                "invoice.lines.data.price.product (expanded)".to_string(),
            ))?;

            lines.push(APIBillingUpcomingInvoiceLine {
                stripe_price_id: price.id().to_string(),
                name: product.name.clone().ok_or(BillingError::FieldNone(
                    "invoice.lines.data.price.product.name".to_string(),
                ))?,
                quantity: line.quantity.ok_or(BillingError::FieldNone(
                    "invoice.lines.data.quantity".to_string(),
                ))?,
                unit_price: price.unit_amount.ok_or(BillingError::FieldNone(
                    "invoice.lines.data.price.unit_amount".to_string(),
                ))?,
                unit_price_per: price.transform_quantity.map(|v| v.divide_by).unwrap_or(1),
                total_price: line.amount,
                proration: line.proration,
                stripe_description: line.description.unwrap_or("".to_string()),
            });
        }

        let mut promotions = Vec::<APIBillingUpcomingInvoicePromotion>::new();
        for discount in value
            .discounts
            .ok_or(BillingError::FieldNone("invoice.discounts".to_string()))?
        {
            let discount = discount.as_object().ok_or(BillingError::FieldNone(
                "invoice.discounts[i] (expanded)".to_string(),
            ))?;

            promotions.push(APIBillingUpcomingInvoicePromotion {
                name: discount.coupon.name.clone().ok_or(BillingError::FieldNone(
                    "invoice.discounts[i].coupon.name".to_string(),
                ))?,
                percent_off: discount.coupon.percent_off,
                amount_off: discount.coupon.amount_off,
            })
        }

        Ok(APIBillingUpcomingInvoice {
            invoice_date: stripe_timestamp_to_chrono(value.created, "invoice.created")?,
            amount_due: value
                .amount_due
                .ok_or(BillingError::FieldNone("invoice.amount_due".to_string()))?,
            tax_amount: value.tax.unwrap_or(0),
            total_amount: value
                .total
                .ok_or(BillingError::FieldNone("invoice.total_amount".to_string()))?,
            starting_balance: value.starting_balance.unwrap_or(0),
            ending_balance: value.ending_balance.unwrap_or(0),
            currency: value
                .currency
                .ok_or(BillingError::FieldNone("invoice.currency".to_string()))?
                .to_string(),
            lines,
            period_start: stripe_timestamp_to_chrono(value.period_start, "invoice.period_start")?,
            period_end: stripe_timestamp_to_chrono(value.period_end, "invoice.period_end")?,
            next_payment_attempt: value
                .next_payment_attempt
                .map(|v| {
                    DateTime::<Utc>::from_timestamp(v, 0).ok_or(BillingError::FieldNone(
                        "invoice.next_payment_attempt invalid date".to_string(),
                    ))
                })
                .unwrap_or(Ok(Utc::now()))?,
            promotions,
        })
    }
}
