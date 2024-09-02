use std::str::FromStr;

use lettre::{address::AddressError, message::Mailbox};
use palform_tsid::{resources::IDUnknown, tsid::PalformDatabaseID};
use thiserror::Error;

use crate::config::Config;

use super::client::PalformMailClient;

pub struct EventNotficationManager;

#[derive(Error, Debug)]
pub enum EventNotificationError {
    #[error("Parse address: {0}")]
    Address(#[from] AddressError),
    #[error("Build message: {0}")]
    Message(#[from] lettre::error::Error),
    #[error("Send message: {0}")]
    Send(#[from] lettre::transport::smtp::Error),
}

impl EventNotficationManager {
    pub async fn notify_event(
        mail: &PalformMailClient,
        config: &Config,
        event_headline: String,
        event_resource_id: PalformDatabaseID<IDUnknown>,
    ) -> Result<(), EventNotificationError> {
        let m = mail
            .get_email_builder()
            .to(Mailbox::from_str(
                config.event_notification_address.as_str(),
            )?)
            .subject(format!("Palform event: {}", event_headline))
            .body(event_resource_id.to_string())?;

        mail.send_email(m).await?;
        Ok(())
    }
}
