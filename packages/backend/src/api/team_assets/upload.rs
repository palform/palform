use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::team_asset::APITeamAsset,
    auth::rbac::requests::APITokenTeamEditorFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct TeamAssetsUploadPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[derive(MultipartForm)]
pub(crate) struct TeamAssetsUploadForm {
    file: TempFile,
}

#[api_operation(
    skip_args = "form",
    tag = "Team Assets",
    operation_id = "organisation.team.asset.upload"
)]
pub async fn team_assets_upload(
    path: Path<TeamAssetsUploadPath>,
    form: MultipartForm<TeamAssetsUploadForm>,
    _token: APITokenTeamEditorFromTeam,
    s3_client: Data<PalformS3Client<S3BucketTeamAssets>>,
    db: Data<DatabaseConnection>,
) -> Result<Json<APITeamAsset>, APIError> {
    let m = TeamAssetsManager::new(path.team_id);
    let mut async_file = tokio::fs::File::from_std(form.0.file.file.into_file());

    let content_type = form
        .0
        .file
        .content_type
        .clone()
        .ok_or(APIError::BadRequest(
            "Could not recognise file type".to_string(),
        ))?;

    let created_team_asset = m
        .create(
            db.as_ref(),
            s3_client.as_ref(),
            &mut async_file,
            &content_type.to_string(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("upload asset", e))?;

    Ok(Json(created_team_asset))
}
