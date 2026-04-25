use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmission},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::submission::{APISubmission, APISubmissionStream},
    auth::rbac::requests::APITokenTeamViewerFromForm,
    crypto::submissions::SubmissionConversionError,
    entity_managers::{forms::FormManager, submission::SubmissionManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub(crate) struct SubmissionsListQuery {
    since: Option<PalformDatabaseID<IDSubmission>>,
}

#[api_operation(tag = "Submissions", operation_id = "submissions.list")]
pub async fn submissions_list(
    path: Path<SubmissionsListPath>,
    Query(query): Query<SubmissionsListQuery>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<APISubmissionStream>, APIError> {
    if !FormManager::verify_form_org(db.as_ref(), path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let submissions = SubmissionManager::list_submissions(db.as_ref(), path.form_id, query.since)
        .await
        .map_err(|e| e.to_internal_error())?;

    let submissions: Result<Vec<APISubmission>, SubmissionConversionError> = submissions
        .iter()
        .map(|s| APISubmission::try_from(s.clone()))
        .collect();
    let submissions =
        submissions.map_err(|e| APIError::report_internal_error("Creating PEM strings", e))?;

    let deleted_submissions =
        SubmissionManager::list_deleted_submissions(db.as_ref(), path.form_id)
            .await
            .map_internal_error()?;

    let total_count = SubmissionManager::count_for_form(db.as_ref(), path.form_id)
        .await
        .map_internal_error()?;

    Ok(Json(APISubmissionStream {
        new: submissions,
        deleted: deleted_submissions,
        since: query.since,
        total: total_count,
    }))
}
