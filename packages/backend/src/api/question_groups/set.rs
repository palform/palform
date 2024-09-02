use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_tsid::{resources::{IDForm, IDOrganisation, IDQuestionGroup}, tsid::PalformDatabaseID};
use rocket::{put, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIErrorWithStatus, APIInternalError},
    auth::rbac::requests::APITokenTeamEditorFromForm,
    entity_managers::{forms::FormManager, question_groups::QuestionGroupManager},
};

#[openapi(tag = "Question Groups", operation_id = "question_groups.set")]
#[put(
    "/users/me/orgs/<org_id>/forms/<form_id>/groups/<question_group_id>",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    data: Json<APIQuestionGroup>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    if !FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !QuestionGroupManager::verify_question_group_form(db.inner(), question_group_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if data.id != question_group_id {
        return Err(APIError::BadRequest("ID in data must match ID in path".to_string()).into());
    }

    QuestionGroupManager::update(db.inner(), data.0)
        .await
        .map_err(|e| e.to_internal_error())?;
    Ok(())
}
