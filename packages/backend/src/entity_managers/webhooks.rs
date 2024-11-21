use palform_entities::{prelude::*, webhook, webhook_job};
use palform_migration::all;
use palform_tsid::{
    resources::{IDForm, IDWebhook},
    tsid::PalformDatabaseID,
};
use rand::Rng;
use rocket::tokio::{net::TcpStream, time::timeout};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, Set,
};
use std::time::Duration;
use thiserror::Error;
use url::{Host, Url};

use crate::api_entities::webhook::APIWebhook;

pub struct WebhookManager;

#[derive(Error, Debug)]
pub enum URLValidationError {
    #[error("URL scheme must be https")]
    WrongScheme,
    #[error("URL did not have a host")]
    NoHost,
    #[error("That host is not allowed")]
    HostNotAllowed,
}

#[derive(Error, Debug)]
pub enum TestConnectionError {
    #[error("did not contain socket addresses: {0}")]
    SocketAddrs(std::io::Error),
    #[error("host took more than 5 seconds to respond")]
    Timeout,
    #[error("failed to contact host: {0}")]
    Connect(std::io::Error),
}

impl WebhookManager {
    pub async fn verify_form<T: ConnectionTrait>(
        conn: &T,
        webhook_id: PalformDatabaseID<IDWebhook>,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<bool, DbErr> {
        Webhook::find_by_id(webhook_id)
            .filter(webhook::Column::FormId.eq(form_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn list<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Vec<APIWebhook>, sea_orm::DbErr> {
        let webhooks: Vec<(webhook::Model, Vec<webhook_job::Model>)> = Webhook::find()
            .filter(webhook::Column::FormId.eq(form_id))
            .find_with_related(WebhookJob)
            .all(conn)
            .await?;

        Ok(webhooks
            .iter()
            .map(|(w, jobs)| {
                let is_healthy = jobs.iter().all(|job| job.error.is_none());

                APIWebhook {
                    id: w.id,
                    form_id: w.form_id,
                    endpoint: w.endpoint.clone(),
                    created_at: w.created_at,
                    is_healthy,
                }
            })
            .collect())
    }

    fn create_signing_secret() -> String {
        let mut arr = [0u8; 64];
        rand::thread_rng().fill(&mut arr);
        faster_hex::hex_string(&arr)
    }

    pub fn validate_url(endpoint: Url) -> Result<(), URLValidationError> {
        if endpoint.scheme() != "https" {
            return Err(URLValidationError::WrongScheme);
        }

        let host = endpoint.host().ok_or(URLValidationError::NoHost)?;
        match host {
            Host::Domain(domain) => {
                if domain == "localhost" || domain.ends_with(".local") {
                    return Err(URLValidationError::HostNotAllowed);
                }
            }
            Host::Ipv4(ipv4) => {
                if ipv4.is_private()
                    || ipv4.is_loopback()
                    || ipv4.is_multicast()
                    || ipv4.is_documentation()
                    || ipv4.is_broadcast()
                    || ipv4.is_unspecified()
                {
                    return Err(URLValidationError::HostNotAllowed);
                }
            }
            Host::Ipv6(ipv6) => {
                if ipv6.is_unspecified() || ipv6.is_multicast() || ipv6.is_loopback() {
                    return Err(URLValidationError::HostNotAllowed);
                }
            }
        }

        Ok(())
    }

    pub async fn test_connection(endpoint: Url) -> Result<(), TestConnectionError> {
        let addrs = endpoint
            .socket_addrs(|| Some(443))
            .map_err(TestConnectionError::SocketAddrs)?;

        timeout(Duration::from_secs(5), TcpStream::connect(&*addrs))
            .await
            .map_err(|_| TestConnectionError::Timeout)?
            .map_err(TestConnectionError::Connect)?;

        Ok(())
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        endpoint: Url,
    ) -> Result<webhook::Model, DbErr> {
        let new_webhook = webhook::ActiveModel {
            id: Set(PalformDatabaseID::<IDWebhook>::random()),
            form_id: Set(form_id),
            endpoint: Set(endpoint.to_string()),
            signing_secret: Set(Self::create_signing_secret()),
            ..Default::default()
        };

        new_webhook.insert(conn).await
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        webhook_id: PalformDatabaseID<IDWebhook>,
    ) -> Result<(), DbErr> {
        Webhook::delete_many()
            .filter(all![
                webhook::Column::Id.eq(webhook_id),
                webhook::Column::FormId.eq(form_id)
            ])
            .exec(conn)
            .await?;

        Ok(())
    }
}
