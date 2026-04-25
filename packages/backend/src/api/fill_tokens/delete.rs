use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::{APIError, APIInternalError},
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        fill_access::FillAccessTokenManager, rbac::requests::APITokenTeamEditorFromForm,
        tokens::APIAuthTokenSource,
    },
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, ApiComponent, JsonSchema)]
pub struct FillTokensDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    token_id: PalformDatabaseID<IDFillAccessToken>,
}

#[api_operation(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.delete")]
pub async fn fill_tokens_delete(
    path: Path<FillTokensDeletePath>,
    token: APITokenTeamEditorFromForm,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !FillAccessTokenManager::verify_token_form(&txn, path.token_id, path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    FillAccessTokenManager::delete(&txn, path.token_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
            Some(format!("Deleted Fill Access Token {}", path.token_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
