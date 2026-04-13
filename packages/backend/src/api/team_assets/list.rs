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
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::team_assets::TeamAssetsManager,
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct TeamAssetsListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Team Assets", operation_id = "organisation.team.asset.list")]
pub async fn team_assets_list(
    path: Path<TeamAssetsListPath>,
    _token: APITokenTeamViewerFromTeam,
    s3: Data<PalformS3Client<S3BucketTeamAssets>>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APITeamAsset>>, APIError> {
    let m = TeamAssetsManager::new(path.team_id);
    let assets = m
        .list(db.as_ref(), s3.as_ref())
        .await
        .map_err(|e| APIError::report_internal_error("list assets", e))?;

    Ok(Json(assets))
}
