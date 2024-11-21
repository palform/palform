use std::collections::{hash_map::Entry, HashMap};

use chrono::Utc;
use palform_entities::{form, prelude::*, submission, webhook, webhook_job};
use palform_tsid::{
    resources::{IDSubmission, IDWebhook, IDWebhookJob},
    tsid::PalformDatabaseID,
};
use ring::hmac;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set, StreamTrait,
};
use thiserror::Error;

use crate::{
    api_entities::{
        submission::{APISubmission, APISubmissionWebhookPayload},
        webhook::APIWebhookJob,
    },
    crypto::submissions::SubmissionConversionError,
};

use super::submission::SubmissionManager;

pub struct WebhookJobsManager<'a, T: ConnectionTrait + StreamTrait> {
    webhooks_cache: HashMap<PalformDatabaseID<IDWebhook>, webhook::Model>,
    conn: &'a T,
}

#[derive(Error, Debug)]
pub enum WebhookJobError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Parsing submission: {0}")]
    SubmissionConversion(#[from] SubmissionConversionError),
    #[error("Serialize payload: {0}")]
    Serialize(#[from] serde_json::Error),
    #[error("Sending request: {0}")]
    Http(#[from] reqwest::Error),
    #[error("Non-success response: {0}")]
    ServerError(String),
}

impl<'a, T: ConnectionTrait + StreamTrait> WebhookJobsManager<'a, T> {
    pub fn new(conn: &'a T) -> Self {
        Self {
            webhooks_cache: HashMap::new(),
            conn,
        }
    }

    pub async fn list(
        &self,
        webhook_id: PalformDatabaseID<IDWebhook>,
    ) -> Result<Vec<APIWebhookJob>, DbErr> {
        WebhookJob::find()
            .filter(webhook_job::Column::WebhookId.eq(webhook_id))
            .order_by(webhook_job::Column::CreatedAt, Order::Desc)
            .into_model()
            .all(self.conn)
            .await
    }

    pub async fn create(
        &self,
        submission_id: PalformDatabaseID<IDSubmission>,
    ) -> Result<(), DbErr> {
        let webhooks = Webhook::find()
            .join(JoinType::LeftJoin, webhook::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Submission.def())
            .filter(submission::Column::Id.eq(submission_id))
            .all(self.conn)
            .await?;

        for webhook in webhooks {
            let new_job = webhook_job::ActiveModel {
                id: Set(PalformDatabaseID::<IDWebhookJob>::random()),
                webhook_id: Set(webhook.id),
                submission_id: Set(submission_id),
                ..Default::default()
            };

            new_job.insert(self.conn).await?;
        }

        Ok(())
    }

    pub async fn run_all_jobs(&mut self) -> Result<(), DbErr> {
        let mut all_jobs = WebhookJob::find()
            .filter(webhook_job::Column::DoneAt.is_null())
            .order_by(webhook_job::Column::CreatedAt, Order::Asc)
            .paginate(self.conn, 100);

        while let Some(jobs) = all_jobs.fetch_and_next().await? {
            for job in jobs {
                let result = self.run_job(&job).await;

                let mut new_job = webhook_job::ActiveModel {
                    id: Set(job.id),
                    ..Default::default()
                };

                match result {
                    Ok(_) => new_job.done_at = Set(Some(Utc::now().naive_utc())),
                    Err(e) => {
                        new_job.done_at = Set(None);
                        new_job.retries = Set(job.retries + 1);
                        new_job.error = Set(Some(e.to_string()));
                    }
                }

                new_job.update(self.conn).await?;
            }
        }

        Ok(())
    }

    async fn get_webhook(
        &mut self,
        webhook_id: PalformDatabaseID<IDWebhook>,
    ) -> Result<webhook::Model, DbErr> {
        if let Entry::Vacant(e) = self.webhooks_cache.entry(webhook_id) {
            let webhook = Webhook::find_by_id(webhook_id)
                .one(self.conn)
                .await?
                .ok_or(DbErr::RecordNotFound("webhook not found".to_string()))?;
            e.insert(webhook);
        }

        let cached_webhook = self
            .webhooks_cache
            .get(&webhook_id)
            .expect("cache to contain webhook that was just inserted");
        Ok(cached_webhook.to_owned())
    }

    fn sign_payload(secret: String, payload: &[u8]) -> String {
        let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
        faster_hex::hex_string(hmac::sign(&key, payload).as_ref())
    }

    async fn run_job(&mut self, job: &webhook_job::Model) -> Result<(), WebhookJobError> {
        let webhook = self.get_webhook(job.webhook_id).await?;
        let submission = SubmissionManager::get_by_id(self.conn, job.submission_id)
            .await?
            .ok_or(WebhookJobError::DBError(DbErr::RecordNotFound(
                "Submission".to_string(),
            )))?;
        let submission = APISubmission::try_from(submission)?;

        let payload = APISubmissionWebhookPayload {
            submission_id: submission.id,
            form_id: webhook.form_id,
            created_at: submission.created_at,
            payload: submission.data,
        };

        let payload = serde_json::to_vec(&payload)?;
        let signature = Self::sign_payload(webhook.signing_secret, &payload);

        let http_client = reqwest::Client::new();
        let resp = http_client
            .post(webhook.endpoint)
            .body(payload)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("x-palform-signature", signature)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            return Err(WebhookJobError::ServerError(status.to_string()));
        }

        Ok(())
    }
}
