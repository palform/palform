use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{forms::FormManager, questions::QuestionManager},
};

#[openapi(tag = "Questions", operation_id = "questions.list")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/groups/all/questions")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIQuestion>>, (Status, Json<APIError>)> {
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

    let questions = QuestionManager::get_all_for_form(&txn, form_id)
        .await
        .map_err(|e| APIError::report_internal_error("get questions", e))?;

    Ok(Json(questions))
}
