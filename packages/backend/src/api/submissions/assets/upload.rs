use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmissionFile},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    auth::fill_access::APIFillAccessToken,
    palform_s3::{buckets::S3BucketSubmissionAssets, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionsAssetsUploadPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(MultipartForm)]
pub(crate) struct SubmissionAssetsUploadRequest {
    #[multipart(rename = "encrypted")]
    file: TempFile,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct SubmissionAssetsUploadResponse {
    file_id: PalformDatabaseID<IDSubmissionFile>,
}

#[api_operation(
    skip_args = "data",
    tag = "Submission Asset",
    operation_id = "submission.assets.upload"
)]
pub async fn submissions_assets_upload(
    path: Path<SubmissionsAssetsUploadPath>,
    data: MultipartForm<SubmissionAssetsUploadRequest>,
    _token: APIFillAccessToken,
    s3: Data<PalformS3Client<S3BucketSubmissionAssets>>,
) -> Result<Json<SubmissionAssetsUploadResponse>, APIError> {
    let mut async_file = tokio::fs::File::from_std(data.0.file.file.into_file());
    let file_id = PalformDatabaseID::<IDSubmissionFile>::random();

    s3.bucket
        // we expect the file to be encrypted (a raw binary stream) so we don't care about
        // content-type
        .put_object_stream(&mut async_file, format!("{}/{}", path.form_id, file_id))
        .await
        .map_err(|e| APIError::report_internal_error("upload submission asset to s3", e))?;

    Ok(Json(SubmissionAssetsUploadResponse { file_id }))
}
