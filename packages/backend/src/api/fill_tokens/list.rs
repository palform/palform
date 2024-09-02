use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::fill_token::APIFillToken,
    auth::{fill_access::FillAccessTokenManager, rbac::requests::APITokenTeamViewerFromForm},
    entity_managers::forms::FormManager,
};

#[openapi(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.list")]
#[get("/users/me/orgs/<org_id>/forms/<form_id>/fill_access_tokens")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFillToken>>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let is_valid = FormManager::verify_form_org(&txn, form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    if !is_valid {
        return Err(APIError::NotAllowed.into());
    }

    let tokens = FillAccessTokenManager::list_for_form(&txn, form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(tokens))
}
