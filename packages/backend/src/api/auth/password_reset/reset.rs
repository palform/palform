use palform_client_common::errors::error::{
    APIError, APIErrorWithStatus, APIInternalError, APIInternalErrorResult,
};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use palform_tsid::{resources::IDAdminUserEmailVerification, tsid::PalformDatabaseID};
use rocket::{put, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::{entity_managers::{
    admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
}, rocket_util::validated::Validated};

#[derive(Deserialize, JsonSchema, Validate)]
pub struct ResetPasswordRequest {
    verification_id: PalformDatabaseID<IDAdminUserEmailVerification>,
    #[validate(length(min = 12, max = 64, message = "must be between 12 and 64 characters"))]
    new_password: String,
}

#[openapi(tag = "Password Resets", operation_id = "user.password_reset.reset")]
#[put("/auth/reset/password", data = "<data>")]
pub async fn handler(
    data: Validated<Json<ResetPasswordRequest>>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let verification = EmailVerificationManager::process_verification(&txn, data.verification_id)
        .await
        .map_err(|e| match e {
            DbErr::RecordNotFound(_) => APIError::BadRequest(
                "That password reset session has expired or doesn't exist. Please restart the process".to_string()
            ).into(),
            _ => e.to_internal_error(),
        })?;

    if verification.purpose != AdminUserEmailVerificationPurposeEnum::PasswordReset {
        return Err(APIError::NotFound.into());
    }

    AdminUserManager::set_user_password(&txn, verification.user_id, data.new_password.clone())
        .await
        .map_err(|e| APIError::report_internal_error("update user password", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
