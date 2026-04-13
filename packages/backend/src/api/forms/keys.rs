use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use sequoia_openpgp::packet::key::PublicParts;
use serde::{Deserialize, Serialize};

use crate::{
    api::error::{APIError, APIInternalError},
    auth::fill_access::APIFillAccessToken,
    crypto::keys::{CryptoKeyRepr, KeyConversionError},
    entity_managers::{forms::FormManager, keys::UserKeyManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsKeysPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct FormsKeysResponse {
    keys: Vec<String>,
}

#[api_operation(tag = "Forms", operation_id = "forms.keys")]
pub async fn forms_keys(
    path: Path<FormsKeysPath>,
    fill_access_token: APIFillAccessToken,
    db: Data<DatabaseConnection>,
) -> Result<Json<FormsKeysResponse>, APIError> {
    if fill_access_token.form_id != path.form_id {
        return Err(APIError::BadRequest("Token must match form ID".to_string()).into());
    }

    let is_valid = FormManager::verify_form_org(db.as_ref(), path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    if !is_valid {
        return Err(APIError::BadRequest("Form does not belong to org".to_string()).into());
    }

    let team_id = FormManager::get_form_team_id(db.as_ref(), path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    let models = UserKeyManager::list_all_team_keys(db.as_ref(), path.org_id, team_id)
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
    Ok(Json(FormsKeysResponse { keys }))
}
