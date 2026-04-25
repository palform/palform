use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_tsid::{
    resources::{IDForm, IDWebhook, IDWebhookJob},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, Clone, ApiComponent)]
pub struct APIWebhook {
    pub id: PalformDatabaseID<IDWebhook>,
    pub form_id: PalformDatabaseID<IDForm>,
    pub endpoint: String,
    pub created_at: DateTime<Utc>,
    pub is_healthy: bool,
}

#[derive(Serialize, JsonSchema, Clone, FromQueryResult, ApiComponent)]
pub struct APIWebhookJob {
    pub id: PalformDatabaseID<IDWebhookJob>,
    pub done_at: Option<DateTime<Utc>>,
    pub retries: Option<i32>,
    pub error: Option<String>,
}
