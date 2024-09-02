use serde::{Deserialize, Serialize};
use stripe::{
    Currency, CustomerId, Discount, Expandable, Invoice, InvoiceLineItem, List, Response,
    Scheduled, SubscriptionId, UpdateSubscriptionItems,
};

#[derive(Clone, Debug, Serialize, Default)]
pub struct UpcomingInvoiceRequest {
    pub customer: CustomerId,
    pub subscription: SubscriptionId,
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePreviewInvoiceAutomaticTax {
    pub enabled: bool,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePreviewInvoiceSubscriptionDetails {
    pub cancel_at_period_end: Option<bool>,
    pub items: Vec<UpdateSubscriptionItems>,
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct CreatePreviewInvoiceRequest {
    pub subscription: SubscriptionId,
    pub automatic_tax: CreatePreviewInvoiceAutomaticTax,
    pub subscription_details: CreatePreviewInvoiceSubscriptionDetails,
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct UpcomingInvoice {
    pub lines: Option<List<InvoiceLineItem>>,
    pub created: Option<i64>,
    pub amount_due: Option<i64>,
    pub total: Option<i64>,
    pub starting_balance: Option<i64>,
    pub ending_balance: Option<i64>,
    pub tax: Option<i64>,
    pub currency: Option<Currency>,
    pub period_start: Option<i64>,
    pub period_end: Option<i64>,
    pub next_payment_attempt: Option<i64>,
    pub discounts: Option<Vec<Expandable<Discount>>>,
}

impl From<UpcomingInvoice> for Invoice {
    fn from(value: UpcomingInvoice) -> Self {
        Self {
            lines: value.lines,
            created: value.created,
            amount_due: value.amount_due,
            total: value.total,
            starting_balance: value.starting_balance,
            ending_balance: value.ending_balance,
            tax: value.tax,
            currency: value.currency,
            period_start: value.period_start,
            period_end: value.period_end,
            next_payment_attempt: value.next_payment_attempt,
            discounts: value.discounts,
            ..Default::default()
        }
    }
}

pub struct InvoiceOverride;
impl InvoiceOverride {
    pub fn upcoming(client: &stripe::Client, request: UpcomingInvoiceRequest) -> Response<Invoice> {
        client.get_query("/invoices/upcoming", request)
    }

    pub fn create_preview(
        client: &stripe::Client,
        request: CreatePreviewInvoiceRequest,
    ) -> Response<UpcomingInvoice> {
        client.post_form("/invoices/create_preview", request)
    }
}
