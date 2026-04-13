use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::fill_token::APIFillToken,
    auth::{fill_access::FillAccessTokenManager, rbac::requests::APITokenTeamViewerFromForm},
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct FillTokensListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.list")]
pub async fn fill_tokens_list(
    path: Path<FillTokensListPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFillToken>>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let is_valid = FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    if !is_valid {
        return Err(APIError::NotAllowed.into());
    }

    let tokens = FillAccessTokenManager::list_for_form(&txn, path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(tokens))
}
