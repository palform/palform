use palform_tsid::{resources::{IDFillAccessToken, IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    auth::{fill_access::FillAccessTokenManager, rbac::requests::APITokenTeamEditorFromForm},
    entity_managers::forms::FormManager,
};

#[openapi(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.delete")]
#[delete("/users/me/orgs/<org_id>/forms/<form_id>/fill_access_tokens/<token_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    token_id: PalformDatabaseID<IDFillAccessToken>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !FillAccessTokenManager::verify_token_form(&txn, token_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !FormManager::verify_form_org(&txn, form_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    FillAccessTokenManager::delete(&txn, token_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
