use chrono::NaiveDateTime;
use palform_tsid::{
    resources::{IDForm, IDWebhook, IDWebhookJob},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, Clone)]
pub struct APIWebhook {
    pub id: PalformDatabaseID<IDWebhook>,
    pub form_id: PalformDatabaseID<IDForm>,
    pub endpoint: String,
    pub created_at: NaiveDateTime,
    pub is_healthy: bool,
}

#[derive(Serialize, JsonSchema, Clone, FromQueryResult)]
pub struct APIWebhookJob {
    pub id: PalformDatabaseID<IDWebhookJob>,
    pub done_at: Option<NaiveDateTime>,
    pub retries: Option<i32>,
    pub error: Option<String>,
}
