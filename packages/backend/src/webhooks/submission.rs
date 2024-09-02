use crate::api_entities::submission::APISubmissionWebhookPayload;

use super::webhook::WebhookManager;

pub struct SubmissionWebhookManager {
    url: url::Url,
    payload: APISubmissionWebhookPayload,
}

impl SubmissionWebhookManager {
    pub fn new(url: &str, payload: APISubmissionWebhookPayload) -> Result<Self, url::ParseError> {
        let url = url::Url::parse(url)?;
        Ok(Self { url, payload })
    }
}

impl WebhookManager<APISubmissionWebhookPayload> for SubmissionWebhookManager {
    fn target_url(&self) -> url::Url {
        self.url.clone()
    }
    fn target_method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }
    fn request_body(&self) -> Option<APISubmissionWebhookPayload> {
        Some(self.payload.clone())
    }
}
