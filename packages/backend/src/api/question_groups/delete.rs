use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIErrorWithStatus, APIInternalError},
    auth::rbac::requests::APITokenTeamEditorFromForm,
    entity_managers::{forms::FormManager, question_groups::QuestionGroupManager},
};

#[openapi(tag = "Question Groups", operation_id = "question_groups.delete")]
#[delete("/users/me/orgs/<org_id>/forms/<form_id>/groups/<question_group_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    QuestionGroupManager::verify_question_group_form(db.inner(), question_group_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    QuestionGroupManager::delete(db.inner(), question_group_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(())
}
