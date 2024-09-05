use palform_client_common::form_management::form_end::APIFormEndConfiguration;
use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::resources::{IDForm, IDFormBranding, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

use super::form_brandings::APIFormBranding;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIForm {
    pub id: PalformDatabaseID<IDForm>,
    pub title: Option<String>,
    pub editor_name: String,
    pub response_count: i64,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    pub notification_email: bool,
    pub notification_webhook_url: Option<String>,
    pub auto_delete_submission_after_days: Option<i32>,
    pub end_configuration: APIFormEndConfiguration,
    pub enable_captcha: bool,
    pub one_question_per_page: bool,
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
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

#[derive(Serialize, JsonSchema)]
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
