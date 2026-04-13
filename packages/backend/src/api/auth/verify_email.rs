use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use palform_tsid::{resources::IDAdminUserEmailVerification, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::entity_managers::{
    admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct VerifyEmailPath {
    verification_id: PalformDatabaseID<IDAdminUserEmailVerification>,
}

#[api_operation(tag = "Authentication", operation_id = "auth.verify")]
pub async fn auth_verify_email(
    path: Path<VerifyEmailPath>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let verification = EmailVerificationManager::process_verification(&txn, path.verification_id)
        .await
        .map_err(|e| match e {
            DbErr::RecordNotFound(_) => APIError::NotFound.into(),
            e => APIError::report_internal_error("process email verification", e),
        })?;

    if verification.purpose != AdminUserEmailVerificationPurposeEnum::NewEmail {
        return Err(APIError::NotFound.into());
    }

    AdminUserManager::mark_user_email_verified(&txn, verification.user_id)
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
