use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmission},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::{APIError, APIInternalError},
    audit::{id_chains::IDChainSubmission, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamEditorFromForm, tokens::APIAuthTokenSource},
    entity_managers::{forms::FormManager, submission::SubmissionManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    submission_id: PalformDatabaseID<IDSubmission>,
}

#[api_operation(tag = "Submissions", operation_id = "submissions.delete")]
pub async fn submissions_delete(
    path: Path<SubmissionsDeletePath>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainSubmission>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !SubmissionManager::verify_submission_form(&txn, path.submission_id, path.form_id)
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

    SubmissionManager::delete_submission(&txn, path.submission_id, path.form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_id_chain(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Submission,
            Some(path.submission_id.into_unknown()),
            Some(IDChainSubmission::new(path.form_id)),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
