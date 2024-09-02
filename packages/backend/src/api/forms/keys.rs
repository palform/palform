use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::PublicParts;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::fill_access::APIFillAccessToken,
    crypto::keys::{CryptoKeyRepr, KeyConversionError},
    entity_managers::{forms::FormManager, keys::UserKeyManager},
};

#[openapi(tag = "Forms", operation_id = "forms.keys")]
#[get("/fill/orgs/<org_id>/forms/<form_id>/keys")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    fill_access_token: APIFillAccessToken,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<String>>, (Status, Json<APIError>)> {
    if fill_access_token.form_id != form_id {
        return Err(APIError::BadRequest("Token must match form ID".to_string()).into());
    }

    let is_valid = FormManager::verify_form_org(db.inner(), form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    if !is_valid {
        return Err(APIError::BadRequest("Form does not belong to org".to_string()).into());
    }

    let team_id = FormManager::get_form_team_id(db.inner(), form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    let models = UserKeyManager::list_all_team_keys(db.inner(), org_id, team_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    let keys: Result<Vec<String>, KeyConversionError> = models
        .iter()
        .map(|e| -> Result<String, KeyConversionError> {
            let repr = CryptoKeyRepr::<PublicParts>::from_database_bytes(&e.public_key)?;
            repr.to_pem_string()
        })
        .collect();

    let keys = keys.map_err(|e| APIError::report_internal_error("converting org keys", e))?;
    Ok(Json(keys))
}
