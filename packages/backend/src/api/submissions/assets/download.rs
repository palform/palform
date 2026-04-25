use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmissionFile},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromForm,
    palform_s3::{buckets::S3BucketSubmissionAssets, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionsAssetsDownloadPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    file_id: PalformDatabaseID<IDSubmissionFile>,
}

#[api_operation(tag = "Submission Asset", operation_id = "submission.assets.get_link")]
pub async fn submissions_assets_download(
    path: Path<SubmissionsAssetsDownloadPath>,
    _token: APITokenTeamViewerFromForm,
    s3: Data<PalformS3Client<S3BucketSubmissionAssets>>,
) -> Result<Vec<u8>, APIError> {
    let resp = s3
        .bucket
        .get_object(format!("{}/{}", path.form_id, path.file_id))
        .await
        .map_err(|e| {
            APIError::report_internal_error("sign download url for submission asset", e)
        })?;

    Ok(resp.to_vec())
}
