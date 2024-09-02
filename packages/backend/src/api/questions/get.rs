use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestion, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{
        forms::FormManager, question_groups::QuestionGroupManager, questions::QuestionManager,
    },
};

#[openapi(tag = "Questions", operation_id = "questions.get")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/groups/<question_group_id>/questions/<question_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    question_id: PalformDatabaseID<IDQuestion>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIQuestion>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .expect("txn");

    if !FormManager::verify_form_org(&txn, form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !QuestionGroupManager::verify_question_group_form(&txn, question_group_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let question = QuestionManager::get_by_id(&txn, question_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;
    if question.group_id != question_group_id {
        return Err(APIError::NotFound.into());
    }

    let api_question = APIQuestion::try_from(question)
        .map_err(|e| APIError::report_internal_error("Decode question config", e))?;

    Ok(Json(api_question))
}
