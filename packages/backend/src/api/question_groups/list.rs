use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIErrorWithStatus, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{forms::FormManager, question_groups::QuestionGroupManager},
};

#[openapi(tag = "Question Groups", operation_id = "question_groups.list")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/groups")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIQuestionGroup>>, APIErrorWithStatus> {
    if !FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let resp = QuestionGroupManager::list_all_for_form(db.inner(), form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(resp))
}
