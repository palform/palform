use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmissionFile},
    tsid::PalformDatabaseID,
};
use rocket::{get, State};
use rocket_okapi::openapi;

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromForm,
    palform_s3::{buckets::S3BucketSubmissionAssets, client::PalformS3Client},
};

#[openapi(tag = "Submission Asset", operation_id = "submission.assets.get_link")]
#[get("/user/me/orgs/<_org_id>/forms/<form_id>/submission-assets/<file_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    file_id: PalformDatabaseID<IDSubmissionFile>,
    _token: APITokenTeamViewerFromForm,
    s3: &State<PalformS3Client<S3BucketSubmissionAssets>>,
) -> Result<Vec<u8>, APIErrorWithStatus> {
    let resp = s3
        .bucket
        .get_object(format!("{}/{}", form_id, file_id))
        .await
        .map_err(|e| {
            APIError::report_internal_error("sign download url for submission asset", e)
        })?;

    Ok(resp.to_vec())
}
