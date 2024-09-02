use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam, IDTeamAsset},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[openapi(tag = "Team Assets", operation_id = "organisation.team.asset.get")]
#[get("/users/me/orgs/<_org_id>/teams/<team_id>/assets/<asset_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    asset_id: PalformDatabaseID<IDTeamAsset>,
    _token: APITokenTeamViewerFromTeam,
    s3: &State<PalformS3Client<S3BucketTeamAssets>>,
    db: &State<DatabaseConnection>,
) -> Result<Json<String>, APIErrorWithStatus> {
    let m = TeamAssetsManager::new(team_id);

    if !m
        .verify_asset_team(db.inner(), asset_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let asset = m
        .get(db.inner(), s3, asset_id)
        .await
        .map_err(|e| APIError::report_internal_error("get team asset for admin", e))?;

    Ok(Json(asset.url))
}
