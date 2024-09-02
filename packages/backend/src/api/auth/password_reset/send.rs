use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    entity_managers::{
        admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
    },
    mail::client::PalformMailClient,
};

#[derive(Deserialize, JsonSchema)]
pub struct SendPasswordResetRequest {
    email: String,
}

#[openapi(tag = "Password Resets", operation_id = "user.password_reset.send")]
#[post("/auth/reset/password/send", data = "<data>")]
pub async fn handler(
    data: Json<SendPasswordResetRequest>,
    db: &State<DatabaseConnection>,
    mail: &State<PalformMailClient>,
) -> Result<(), APIErrorWithStatus> {
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
            mail,
        )
        .await
        .map_err(|e| APIError::report_internal_error("send password reset", e))?;

        txn.commit().await.map_internal_error()?;
        Ok(())
    } else {
        Ok(())
    }
}
