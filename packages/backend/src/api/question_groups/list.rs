use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{forms::FormManager, question_groups::QuestionGroupManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct QuestionGroupsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Question Groups", operation_id = "question_groups.list")]
pub async fn question_groups_list(
    path: Path<QuestionGroupsListPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIQuestionGroup>>, APIError> {
    if !FormManager::verify_form_org(db.as_ref(), path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let resp = QuestionGroupManager::list_all_for_form(db.as_ref(), path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(resp))
}
