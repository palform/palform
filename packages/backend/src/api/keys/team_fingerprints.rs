use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromTeam, entity_managers::keys::UserKeyManager,
};

#[openapi(tag = "Organisation Keys", operation_id = "org.keys.team_fingerprints")]
#[get("/users/me/orgs/<org_id>/teams/<team_id>/keys/all")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    _token: APITokenTeamViewerFromTeam,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<String>>, APIErrorWithStatus> {
    let all_keys: Vec<String> = UserKeyManager::list_all_team_keys(db.inner(), org_id, team_id)
        .await
        .map_internal_error()?
        .iter()
        .map(|k| k.cert_fingerprint.clone())
        .collect();

    Ok(Json(all_keys))
}
