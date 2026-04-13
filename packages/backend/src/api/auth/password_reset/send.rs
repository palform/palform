use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    entity_managers::{
        admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
    },
    mail::client::PalformMailClient,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SendPasswordResetRequest {
    email: String,
}

#[api_operation(tag = "Password Resets", operation_id = "user.password_reset.send")]
pub async fn auth_password_reset_send(
    data: Json<SendPasswordResetRequest>,
    db: Data<DatabaseConnection>,
    mail: Data<PalformMailClient>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let user = AdminUserManager::get_user_by_email(&txn, data.email.to_owned())
        .await
        .map_internal_error()?;

    if let Some(user) = user {
        EmailVerificationManager::send_email_verification(
            &txn,
            user.id,
            Some(user.email),
            AdminUserEmailVerificationPurposeEnum::PasswordReset,
            mail.as_ref(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("send password reset", e))?;

        txn.commit().await.map_internal_error()?;
        Ok(())
    } else {
        Ok(())
    }
}
