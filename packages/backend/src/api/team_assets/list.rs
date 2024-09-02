use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::{IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::team_asset::APITeamAsset,
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[openapi(tag = "Team Assets", operation_id = "organisation.team.asset.list")]
#[get("/users/me/orgs/<_org_id>/teams/<team_id>/assets")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    _token: APITokenTeamViewerFromTeam,
    s3: &State<PalformS3Client<S3BucketTeamAssets>>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APITeamAsset>>, APIErrorWithStatus> {
    let m = TeamAssetsManager::new(team_id);
    let assets = m
        .list(db.inner(), s3)
        .await
        .map_err(|e| APIError::report_internal_error("list assets", e))?;

    Ok(Json(assets))
}
