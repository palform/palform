use std::io::Read;

use actix_web::web::{Bytes, Data};
use apistos::api_operation;
use bytes::Buf;
use palform_client_common::errors::error::APIError;
use sea_orm::DatabaseConnection;
use stripe::{EventObject, EventType, Webhook};

use crate::{billing::webhook::BillingWebhookManager, config::Config};

use super::StripeSignature;

#[api_operation(skip_args = "data", tag = "Billing Webhooks", operation_id = "billing.webhook.receive")]
pub async fn billing_webhooks_receiver_handler(
    data: Bytes,
    signature: StripeSignature,
    config: Data<Config>,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let mut reader = data.reader();
    let mut data_string = String::default();
    reader
        .read_to_string(&mut data_string)
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let event = Webhook::construct_event(&data_string, &signature.0, &config.stripe_webhook_secret)
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let manager = BillingWebhookManager::new(stripe.as_ref(), db.as_ref());

    match event.type_ {
        EventType::CustomerSubscriptionCreated
        | EventType::CustomerSubscriptionDeleted
        | EventType::CustomerSubscriptionPaused
        | EventType::CustomerSubscriptionResumed
        | EventType::CustomerSubscriptionUpdated => {
            if let EventObject::Subscription(s) = event.data.object {
                manager
                    .subscription(event.type_, s)
                    .await
                    .map_err(|e| APIError::BadRequest(e.to_string()))?;
            }
        }
        _ => {
            return Err(APIError::BadRequest("Event not implemented".to_string()).into());
        }
    }

    Ok(())
}
