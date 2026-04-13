use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestion, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{
        forms::FormManager, question_groups::QuestionGroupManager, questions::QuestionManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct QuestionsGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    question_id: PalformDatabaseID<IDQuestion>,
}

#[api_operation(tag = "Questions", operation_id = "questions.get")]
pub async fn questions_get(
    path: Path<QuestionsGetPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIQuestion>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .expect("txn");

    if !FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !QuestionGroupManager::verify_question_group_form(&txn, path.question_group_id, path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let question = QuestionManager::get_by_id(&txn, path.question_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;
    if question.group_id != path.question_group_id {
        return Err(APIError::NotFound.into());
    }

    let api_question = APIQuestion::try_from(question)
        .map_err(|e| APIError::report_internal_error("Decode question config", e))?;

    Ok(Json(api_question))
}
