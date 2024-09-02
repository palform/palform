use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::{IDForm, IDOrganisation, IDTeamAsset}, tsid::PalformDatabaseID};
use rocket::{get, response::Redirect, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::fill_access::APIFillAccessToken,
    entity_managers::{forms::FormManager, team_assets::TeamAssetsManager},
    palform_s3::{buckets::S3BucketTeamAssets, client::PalformS3Client},
};

#[openapi(
    tag = "Team Assets",
    operation_id = "organisation.team.asset.get_for_form_fill"
)]
#[get("/fill/orgs/<_org_id>/forms/<form_id>/assets/<asset_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    asset_id: PalformDatabaseID<IDTeamAsset>,
    _token: APIFillAccessToken,
    s3: &State<PalformS3Client<S3BucketTeamAssets>>,
    db: &State<DatabaseConnection>,
) -> Result<Redirect, APIErrorWithStatus> {
    let form_team = FormManager::get_form_team_id(db.inner(), form_id)
        .await
        .map_internal_error()?;

    let m = TeamAssetsManager::new(form_team);
    let asset = m
        .get(db.inner(), s3, asset_id)
        .await
        .map_err(|e| APIError::report_internal_error("get single team asset", e))?;

    Ok(Redirect::to(asset.url))
}
