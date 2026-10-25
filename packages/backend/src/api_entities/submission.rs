use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::submission;
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDSubmission, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::crypto::submissions::{CryptoSubmissionRepr, SubmissionConversionError};

#[derive(Serialize, JsonSchema, Clone, ApiComponent)]
pub struct APISubmissionStream {
    /// If requested, the date since which the changes cover
    pub since: Option<PalformDatabaseID<IDSubmission>>,
    /// The total number of non-deleted submissions to the form (over all time, regardless of `since`)
    pub total: u64,
    /// New submissions
    pub new: Vec<APISubmission>,
    /// IDs of submissions that were deleted
    pub deleted: Vec<PalformDatabaseID<IDSubmission>>,
}

#[derive(Serialize, JsonSchema, Clone, ApiComponent)]
pub struct APISubmission {
    pub id: PalformDatabaseID<IDSubmission>,
    pub created_at: DateTime<Utc>,
    /// The fill access token that the user used to fill out the form
    pub for_token: Option<PalformDatabaseID<IDFillAccessToken>>,
    /// The submission data as an encrypted PGP message in armored ASCII form. The message itself
    /// is a JSON object of the form `[palform_client_common::form_management::submission::InProgressSubmission]`.
    pub data: String,
}

impl TryFrom<submission::Model> for APISubmission {
    type Error = SubmissionConversionError;
    fn try_from(value: submission::Model) -> Result<Self, Self::Error> {
        let pem_data = CryptoSubmissionRepr::to_pem_string(&value.encrypted_data)?;
        Ok(Self {
            id: value.id,
            created_at: value.created_at.to_utc(),
            for_token: value.for_token,
            data: pem_data,
        })
    }
}

#[derive(Serialize, Clone)]
pub struct APISubmissionWebhookPayload {
    pub submission_id: PalformDatabaseID<IDSubmission>,
    pub form_id: PalformDatabaseID<IDForm>,
    pub created_at: DateTime<Utc>,
    pub payload: String,
}

#[derive(Serialize, JsonSchema, Clone, FromQueryResult, ApiComponent)]
pub struct APISubmissionCountPerForm {
    pub form_id: PalformDatabaseID<IDForm>,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub new_submission_count: i64,
}
