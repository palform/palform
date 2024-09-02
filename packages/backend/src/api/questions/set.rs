use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::resources::{IDForm, IDOrganisation, IDQuestion, IDQuestionGroup};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{put, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::api::error::{APIError, APIErrorWithStatus, APIInternalError};
use crate::auth::rbac::requests::APITokenTeamEditorFromForm;
use crate::entity_managers::question_groups::QuestionGroupManager;
use crate::entity_managers::questions::QuestionManager;

#[openapi(tag = "Questions", operation_id = "questions.set")]
#[put(
    "/users/me/orgs/<_org_id>/forms/<form_id>/groups/<question_group_id>/questions/<question_id>",
    data = "<new_data>"
)]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    question_id: PalformDatabaseID<IDQuestion>,
    new_data: Json<APIQuestion>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIQuestion>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| APIError::report_internal_error("txn", e))?;

    if !QuestionGroupManager::verify_question_group_form(&txn, question_group_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if QuestionManager::check_question_exist(&txn, question_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        if !QuestionManager::verify_question_group(&txn, question_id, question_group_id)
            .await
            .map_err(|e| e.to_internal_error())?
        {
            return Err(APIError::NotFound.into());
        }

        if new_data.id != question_id {
            return Err(APIError::BadRequest(
                "Question ID did not match route question ID".to_string(),
            )
            .into());
        }
    }

    let question = QuestionManager::set_question(&txn, question_group_id, new_data.0)
        .await
        .map_err(|e| APIError::report_internal_error("set question", e))?;

    txn.commit()
        .await
        .map_err(|e| APIError::report_internal_error("txn", e))?;
    Ok(Json(question))
}
