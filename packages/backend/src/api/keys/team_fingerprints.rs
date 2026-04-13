use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    auth::rbac::requests::APITokenTeamViewerFromTeam, entity_managers::keys::UserKeyManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct KeysTeamFingerprints {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct KeysTeamFingerprintsResponse {
    fingerprints: Vec<String>,
}

#[api_operation(tag = "Organisation Keys", operation_id = "org.keys.team_fingerprints")]
pub async fn keys_team_fingerprints(
    path: Path<KeysTeamFingerprints>,
    _token: APITokenTeamViewerFromTeam,
    db: Data<DatabaseConnection>,
) -> Result<Json<KeysTeamFingerprintsResponse>, APIError> {
    let all_keys: Vec<String> =
        UserKeyManager::list_all_team_keys(db.as_ref(), path.org_id, path.team_id)
            .await
            .map_internal_error()?
            .iter()
            .map(|k| k.cert_fingerprint.clone())
            .collect();

    Ok(Json(KeysTeamFingerprintsResponse {
        fingerprints: all_keys,
    }))
}
