use palform_tsid::resources::{IDForm, IDOrganisation, IDQuestionGroup};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIErrorWithStatus, APIInternalError},
    auth::rbac::requests::APITokenTeamEditorFromForm,
    entity_managers::{forms::FormManager, question_groups::QuestionGroupManager},
};

#[derive(Deserialize, JsonSchema)]
pub struct NewQuestionGroupRequest {
    position: i32,
    title: Option<String>,
    description: Option<String>,
}

#[openapi(tag = "Question Groups", operation_id = "question_groups.create")]
#[post("/users/me/orgs/<org_id>/forms/<form_id>/groups", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Json<NewQuestionGroupRequest>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDQuestionGroup>>, APIErrorWithStatus> {
    if !FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let new_id = QuestionGroupManager::create(
        db.inner(),
        form_id,
        data.position.clone(),
        data.title.clone(),
        data.description.clone(),
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    Ok(Json(new_id))
}
