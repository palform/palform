use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::{APIError, APIInternalError},
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamEditorFromForm, tokens::APIAuthTokenSource},
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Forms", operation_id = "forms.delete")]
pub async fn forms_delete(
    path: Path<FormsDeletePath>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if !FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    FormManager::delete(&txn, path.form_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
        )
        .await
        .map_internal_error()?;
    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
