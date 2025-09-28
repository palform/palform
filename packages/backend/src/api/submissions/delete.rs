use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDSubmission},
    tsid::PalformDatabaseID,
};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    audit::AuditManager,
    auth::{rbac::requests::APITokenTeamEditorFromForm, tokens::APIAuthTokenSource},
    entity_managers::{forms::FormManager, submission::SubmissionManager},
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "Submissions", operation_id = "submissions.delete")]
#[delete("/users/me/orgs/<org_id>/forms/<form_id>/submissions/<submission_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    submission_id: PalformDatabaseID<IDSubmission>,
    token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::ReadCommitted),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !SubmissionManager::verify_submission_form(&txn, submission_id, form_id)
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

    SubmissionManager::delete_submission(&txn, submission_id, form_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Submission,
            Some(submission_id.into_unknown()),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
