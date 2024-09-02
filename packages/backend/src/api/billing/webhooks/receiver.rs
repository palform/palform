use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use rocket::{data::ToByteUnit, post, tokio::io::AsyncReadExt, Data, State};
use sea_orm::DatabaseConnection;
use stripe::{EventObject, EventType, Webhook};

use crate::{billing::webhook::BillingWebhookManager, config::Config};

use super::StripeSignature;

#[post("/billing/webhook", data = "<data>")]
pub async fn handler(
    data: Data<'_>,
    signature: StripeSignature,
    config: &State<Config>,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    let mut stream = data.open(5.mebibytes());
    let mut data_string = String::default();
    stream
        .read_to_string(&mut data_string)
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let event = Webhook::construct_event(&data_string, &signature.0, &config.stripe_webhook_secret)
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let manager = BillingWebhookManager::new(stripe.inner(), db.inner());

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
