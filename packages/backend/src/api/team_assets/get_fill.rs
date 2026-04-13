use actix_web::web::{Data, Path, Redirect};
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDTeamAsset},
    tsid::PalformDatabaseID,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    auth::fill_access::APIFillAccessToken,
    entity_managers::{forms::FormManager, team_assets::TeamAssetsManager},
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[derive(Deserialize)]
pub struct TeamAssetsGetFillPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    asset_id: PalformDatabaseID<IDTeamAsset>,
}

#[api_operation(skip, tag = "Team Assets", operation_id = "organisation.team.asset.get_for_form_fill")]
pub async fn team_assets_get_fill(
    path: Path<TeamAssetsGetFillPath>,
    _token: APIFillAccessToken,
    s3: Data<PalformS3Client<S3BucketTeamAssets>>,
    db: Data<DatabaseConnection>,
) -> Result<Redirect, APIError> {
    let form_team = FormManager::get_form_team_id(db.as_ref(), path.form_id)
        .await
        .map_internal_error()?;

    let m = TeamAssetsManager::new(form_team);
    let asset = m
        .get(db.as_ref(), s3.as_ref(), path.asset_id)
        .await
        .map_err(|e| APIError::report_internal_error("get single team asset", e))?;

    Ok(Redirect::to(asset.url))
}
