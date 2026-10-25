use apistos::ApiComponent;
use palform_client_common::form_management::form_end::APIFormEndConfiguration;
use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::resources::{IDForm, IDFormBranding, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

use super::form_brandings::APIFormBranding;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIForm {
    pub id: PalformDatabaseID<IDForm>,
    /// A public-facing name, shown only on non-OQPP forms
    pub title: Option<String>,
    /// A name shown only to the admins and editors of the form
    pub editor_name: String,
    /// An up-to-date count of all non-deleted responses across all fill access tokens
    pub response_count: i64,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    /// The email address to which new response notifications are sent
    pub notification_email: bool,
    /// The URL of the webhook to which new response notifications are sent
    pub notification_webhook_url: Option<String>,
    /// Submissions are automatically deleted this number of days after submission
    pub auto_delete_submission_after_days: Option<i32>,
    pub end_configuration: APIFormEndConfiguration,
    /// Whether to require a captcha to submit the form
    pub enable_captcha: bool,
    /// Whether to use the One Question Per Page (OQPP) flow, where each question is on a different
    /// page and the user has to click "Next" to continue. Otherwise, multiple questions are allowed
    /// per page, and the form editor can decide which pages contain which questions.
    pub one_question_per_page: bool,
}

/// A stripped-down version of the form's metadata shown to non-admin users filling the form
#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIFrontendForm {
    pub id: PalformDatabaseID<IDForm>,
    pub title: Option<String>,
    pub editor_name: String,
    #[serde(skip)]
    pub branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    pub end_configuration: APIFormEndConfiguration,
    pub enable_captcha: bool,
    pub one_question_per_page: bool,
}

/// A full dump of all the information needed to fully display a form, in a compressed format with
/// shortened keys.
#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct APIFormWithQuestions {
    #[serde(rename = "f")]
    pub form: APIFrontendForm,
    #[serde(rename = "q")]
    pub questions: Vec<APIQuestion>,
    #[serde(rename = "g")]
    pub groups: Vec<APIQuestionGroup>,
    #[serde(rename = "b")]
    pub branding: Option<APIFormBranding>,
    #[serde(rename = "o")]
    pub org_name: String,
}
