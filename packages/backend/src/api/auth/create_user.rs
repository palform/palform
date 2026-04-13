use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use validator::Validate;

use crate::actix_util::validated::Validated;
use crate::captcha::VerifiedCaptcha;
use crate::entity_managers::admin_users::AdminUserManager;
use crate::entity_managers::email_verifications::EmailVerificationManager;
use crate::mail::client::PalformMailClient;

#[derive(Debug, Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct CreateUserRequest {
    #[validate(email(message = "was not a valid email"))]
    email: String,
    #[validate(length(min = 12, max = 64, message = "must be between 12 and 64 characters"))]
    password: String,
}

#[api_operation(tag = "Authentication", operation_id = "auth.sign_up")]
pub async fn auth_create_user(
    request: Validated<Json<CreateUserRequest>>,
    db: Data<DatabaseConnection>,
    mail: Data<PalformMailClient>,
    _captcha: VerifiedCaptcha,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if AdminUserManager::get_user_by_email(db.as_ref(), request.email.clone())
        .await
        .map_internal_error()?
        .is_some()
    {
        return Ok(());
    }

    let new_user_id =
        AdminUserManager::create_user(&txn, request.email.clone(), request.password.clone())
            .await
            .map_err(|e| APIError::report_internal_error("create user", e))?;

    EmailVerificationManager::send_email_verification(
        &txn,
        new_user_id,
        Some(request.email.clone()),
        AdminUserEmailVerificationPurposeEnum::NewEmail,
        mail.as_ref(),
    )
    .await
    .map_err(|e| APIError::report_internal_error("send verification email", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
