use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use palform_tsid::{resources::IDAdminUserEmailVerification, tsid::PalformDatabaseID};
use rocket::{post, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};

use crate::entity_managers::{
    admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
};

#[openapi(tag = "Authentication", operation_id = "auth.verify")]
#[post("/auth/verify/<verification_id>", rank = 1)]
pub async fn handler(
    verification_id: PalformDatabaseID<IDAdminUserEmailVerification>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let verification = EmailVerificationManager::process_verification(&txn, verification_id)
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
