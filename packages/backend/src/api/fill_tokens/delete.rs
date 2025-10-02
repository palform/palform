use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    audit::AuditManager,
    auth::{
        fill_access::FillAccessTokenManager, rbac::requests::APITokenTeamEditorFromForm,
        tokens::APIAuthTokenSource,
    },
    entity_managers::forms::FormManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "Fill Access Tokens", operation_id = "fill_access_tokens.delete")]
#[delete("/users/me/orgs/<org_id>/forms/<form_id>/fill_access_tokens/<token_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    token_id: PalformDatabaseID<IDFillAccessToken>,
    token: APITokenTeamEditorFromForm,
    audit: FromOrgId<AuditManager>,
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

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(form_id.into_unknown()),
            Some(format!("Deleted Fill Access Token {}", token_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
