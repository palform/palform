use palform_client_common::form_management::question_types::{
    APIQuestion, APIQuestionConfiguration,
};
use palform_entities::question;
use palform_tsid::{
    resources::{IDQuestion, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Clone)]
pub struct QuestionWithEncodedConfiguration {
    pub id: PalformDatabaseID<IDQuestion>,
    pub title: String,
    pub internal_name: Option<String>,
    pub description: Option<String>,
    pub required: bool,
    pub position: i32,
    pub group_id: PalformDatabaseID<IDQuestionGroup>,
    pub configuration: sea_orm::JsonValue,
}

impl TryFrom<QuestionWithEncodedConfiguration> for APIQuestion {
    type Error = serde_json::Error;
    fn try_from(value: QuestionWithEncodedConfiguration) -> Result<Self, Self::Error> {
        let config: APIQuestionConfiguration = serde_json::from_value(value.configuration)?;
        Ok(Self {
            id: value.id,
            title: value.title,
            internal_name: value.internal_name,
            description: value.description,
            required: value.required,
            position: value.position,
            group_id: value.group_id,
            configuration: config,
        })
    }
}

impl QuestionWithEncodedConfiguration {
    pub fn encode_config(
        config: APIQuestionConfiguration,
    ) -> Result<sea_orm::JsonValue, serde_json::Error> {
        serde_json::to_value(config)
    }
}

impl From<question::Model> for QuestionWithEncodedConfiguration {
    fn from(value: question::Model) -> Self {
        Self {
            id: value.id,
            title: value.title,
            internal_name: value.internal_name,
            description: value.description,
            required: value.required,
            position: value.position,
            group_id: value.group_id,
            configuration: value.configuration,
        }
    }
}
