use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestion, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamEditorFromForm,
    entity_managers::{
        forms::FormManager, question_groups::QuestionGroupManager, questions::QuestionManager,
    },
};

#[openapi(tag = "Questions", operation_id = "questions.delete")]
#[delete(
    "/users/me/orgs/<org_id>/forms/<form_id>/groups/<question_group_id>/questions/<question_id>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    question_id: PalformDatabaseID<IDQuestion>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

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

    if !QuestionManager::verify_question_group(&txn, question_id, question_group_id)
        .await
        .map_err(|e| e.to_internal_error_with_reason("verify question form"))?
    {
        return Err(APIError::NotFound.into());
    }

    QuestionManager::delete_question(&txn, question_id)
        .await
        .map_err(|e| e.to_internal_error_with_reason("delete question"))?;
    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
