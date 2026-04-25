use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::form_management::question_types::APIQuestion;
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{forms::FormManager, questions::QuestionManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct QuestionsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Questions", operation_id = "questions.list")]
pub async fn questions_list(
    path: Path<QuestionsListPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIQuestion>>, APIError> {
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

    let questions = QuestionManager::get_all_for_form(&txn, path.form_id)
        .await
        .map_err(|e| APIError::report_internal_error("get questions", e))?;

    Ok(Json(questions))
}
