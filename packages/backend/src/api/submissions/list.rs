use palform_client_common::errors::error::APIInternalErrorResult;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmission},
    tsid::PalformDatabaseID,
};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::submission::{APISubmission, APISubmissionStream},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    crypto::submissions::SubmissionConversionError,
    entity_managers::{forms::FormManager, submission::SubmissionManager},
};

#[openapi(tag = "Submissions", operation_id = "submissions.list")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/submissions?<since>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    since: Option<PalformDatabaseID<IDSubmission>>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<APISubmissionStream>, (Status, Json<APIError>)> {
    if !FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let submissions = SubmissionManager::list_submissions(db.inner(), form_id, since)
        .await
        .map_err(|e| e.to_internal_error())?;

    let submissions: Result<Vec<APISubmission>, SubmissionConversionError> = submissions
        .iter()
        .map(|s| APISubmission::try_from(s.clone()))
        .collect();
    let submissions =
        submissions.map_err(|e| APIError::report_internal_error("Creating PEM strings", e))?;

    let deleted_submissions = SubmissionManager::list_deleted_submissions(db.inner(), form_id)
        .await
        .map_internal_error()?;

    let total_count = SubmissionManager::count_for_form(db.inner(), form_id)
        .await
        .map_internal_error()?;

    Ok(Json(APISubmissionStream {
        new: submissions,
        deleted: deleted_submissions,
        since,
        total: total_count,
    }))
}
