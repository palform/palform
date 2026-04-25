use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam, IDTeamAsset},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct TeamAssetsGetPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    asset_id: PalformDatabaseID<IDTeamAsset>,
}

#[api_operation(tag = "Team Assets", operation_id = "organisation.team.asset.get")]
pub async fn team_assets_get(
    path: Path<TeamAssetsGetPath>,
    _token: APITokenTeamViewerFromTeam,
    s3: Data<PalformS3Client<S3BucketTeamAssets>>,
    db: Data<DatabaseConnection>,
) -> Result<Json<String>, APIError> {
    let m = TeamAssetsManager::new(path.team_id);

    if !m
        .verify_asset_team(db.as_ref(), path.asset_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let asset = m
        .get(db.as_ref(), s3.as_ref(), path.asset_id)
        .await
        .map_err(|e| APIError::report_internal_error("get team asset for admin", e))?;

    Ok(Json(asset.url))
}
