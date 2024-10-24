use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    entity_managers::{
        admin_users::AdminUserManager, email_verifications::EmailVerificationManager,
    },
    mail::client::PalformMailClient,
};

#[derive(Deserialize, JsonSchema)]
pub struct ResendVerificationRequest {
    email: String,
}

#[openapi(tag = "Authentication", operation_id = "auth.resend_ verification")]
#[post("/users/verification", data = "<request>")]
pub async fn handler(
    request: Json<ResendVerificationRequest>,
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

    let user = AdminUserManager::get_user_by_email(&txn, request.email.clone())
        .await
        .map_internal_error()?;

    if let Some(user) = user {
        if !user.manual_auth_email_verified.is_some_and(|v| !v) {
            return Ok(());
        }

        EmailVerificationManager::send_email_verification(
            &txn,
            user.id,
            Some(request.email.to_owned()),
            AdminUserEmailVerificationPurposeEnum::NewEmail,
            mail,
        )
        .await
        .map_err(|e| APIError::report_internal_error("resend email verification", e))?;
    }

    txn.commit().await.map_internal_error()?;
    Ok(())
}
