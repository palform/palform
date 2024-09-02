use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::{IDForm, IDOrganisation, IDSubmissionFile}, tsid::PalformDatabaseID};
use rocket::{form::Form, fs::TempFile, post, serde::json::Json, tokio::io::AsyncReadExt, State};

use crate::{
    auth::fill_access::APIFillAccessToken,
    palform_s3::{buckets::S3BucketSubmissionAssets, client::PalformS3Client},
};

#[post("/fill/orgs/<_org_id>/forms/<form_id>/assets", data = "<data>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Form<TempFile<'_>>,
    _token: APIFillAccessToken,
    s3: &State<PalformS3Client<S3BucketSubmissionAssets>>,
) -> Result<Json<PalformDatabaseID<IDSubmissionFile>>, APIErrorWithStatus> {
    let mut buf = Vec::new();
    data.open()
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?
        .read_to_end(&mut buf)
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let file_id = PalformDatabaseID::<IDSubmissionFile>::random();

    s3.bucket
        // we expect the file to be encrypted (a raw binary stream) so we don't care about
        // content-type
        .put_object(
            format!("{}/{}", form_id, file_id),
            &buf,
        )
        .await
        .map_err(|e| APIError::report_internal_error("upload submission asset to s3", e))?;

    Ok(Json(file_id))
}
