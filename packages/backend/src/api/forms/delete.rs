use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    audit::AuditManager,
    auth::rbac::requests::APITokenTeamEditorFromForm,
    auth::tokens::APIAuthTokenSource,
    entity_managers::forms::FormManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "Forms", operation_id = "forms.delete")]
#[delete("/users/me/orgs/<org_id>/forms/<form_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if !FormManager::verify_form_org(&txn, form_id, org_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    FormManager::delete(&txn, form_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Form,
            Some(form_id.into_unknown()),
        )
        .await
        .map_internal_error()?;
    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
