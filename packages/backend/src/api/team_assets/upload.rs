use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::{IDOrganisation, IDTeam, IDTeamAsset}, tsid::PalformDatabaseID};
use rocket::{form::Form, fs::TempFile, post, serde::json::Json, tokio::io::AsyncReadExt, State};
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[post("/users/me/orgs/<_org_id>/teams/<team_id>/assets", data = "<data>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    data: Form<TempFile<'_>>,
    _token: APITokenTeamEditorFromTeam,
    s3_client: &State<PalformS3Client<S3BucketTeamAssets>>,
    db: &State<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDTeamAsset>>, APIErrorWithStatus> {
    let m = TeamAssetsManager::new(team_id);
    let mut buf = Vec::new();
    data.open()
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?
        .read_to_end(&mut buf)
        .await
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    let content_type = data.content_type().ok_or(APIError::BadRequest(
        "Could not recognise file type".to_string(),
    ))?;

    let asset_id = m
        .create(db.inner(), &s3_client, &buf, &content_type.to_string())
        .await
        .map_err(|e| APIError::report_internal_error("upload asset", e))?;

    Ok(Json(asset_id))
}
