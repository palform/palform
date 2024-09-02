use palform_client_common::form_management::question_group::{
    APIQuestionGroup, APIQuestionGroupStepStrategy,
};
use palform_tsid::{resources::IDQuestionGroup, tsid::PalformDatabaseID};
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Clone)]
pub struct QuestionGroupWithEncodedStrategy {
    id: PalformDatabaseID<IDQuestionGroup>,
    title: Option<String>,
    description: Option<String>,
    position: i32,
    step_strategy: sea_orm::JsonValue,
}

impl TryFrom<QuestionGroupWithEncodedStrategy> for APIQuestionGroup {
    type Error = serde_json::Error;
    fn try_from(value: QuestionGroupWithEncodedStrategy) -> Result<Self, Self::Error> {
        let parsed_strategy =
            serde_json::from_value::<APIQuestionGroupStepStrategy>(value.step_strategy)?;
        Ok(Self {
            id: value.id,
            position: value.position,
            title: value.title,
            description: value.description,
            step_strategy: parsed_strategy,
        })
    }
}
