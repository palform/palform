use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use palform_tsid::{resources::IDAdminUserEmailVerification, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::{
    actix_util::validated::Validated,
    entity_managers::{
        admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
    },
};

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct ResetPasswordRequest {
    verification_id: PalformDatabaseID<IDAdminUserEmailVerification>,
    #[validate(length(min = 12, max = 64, message = "must be between 12 and 64 characters"))]
    new_password: String,
}

#[api_operation(tag = "Password Resets", operation_id = "user.password_reset.reset")]
pub async fn auth_password_reset_reset(
    data: Validated<Json<ResetPasswordRequest>>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
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
