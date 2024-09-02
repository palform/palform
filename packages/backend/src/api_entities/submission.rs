use chrono::{DateTime, Utc};
use palform_entities::submission;
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDOrganisation, IDSubmission, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::crypto::submissions::{CryptoSubmissionRepr, SubmissionConversionError};

#[derive(Serialize, JsonSchema, Clone)]
pub struct APISubmissionStream {
    pub since: Option<PalformDatabaseID<IDSubmission>>,
    pub total: u64,
    pub new: Vec<APISubmission>,
    pub deleted: Vec<PalformDatabaseID<IDSubmission>>,
}

#[derive(Serialize, JsonSchema, Clone)]
pub struct APISubmission {
    pub id: PalformDatabaseID<IDSubmission>,
    pub created_at: DateTime<Utc>,
    pub for_token: Option<PalformDatabaseID<IDFillAccessToken>>,
    pub data: String,
}

impl TryFrom<submission::Model> for APISubmission {
    type Error = SubmissionConversionError;
    fn try_from(value: submission::Model) -> Result<Self, Self::Error> {
        let pem_data = CryptoSubmissionRepr::to_pem_string(&value.encrypted_data)?;
        Ok(Self {
            id: value.id,
            created_at: value.created_at.and_utc(),
            for_token: value.for_token,
            data: pem_data,
        })
    }
}

#[derive(Serialize, Clone)]
pub struct APISubmissionWebhookPayload {
    pub submission_id: PalformDatabaseID<IDSubmission>,
    pub form_id: PalformDatabaseID<IDForm>,
    pub org_id: PalformDatabaseID<IDOrganisation>,
    pub payload: String,
}

#[derive(Serialize, JsonSchema, Clone, FromQueryResult)]
pub struct APISubmissionCountPerForm {
    pub form_id: PalformDatabaseID<IDForm>,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub new_submission_count: i64,
}
