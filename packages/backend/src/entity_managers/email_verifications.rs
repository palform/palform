use chrono::{Duration, Utc};
use lettre::message::{header::ContentType, Mailbox};
use palform_entities::{
    admin_user, admin_user_email_verification, prelude::*,
    sea_orm_active_enums::AdminUserEmailVerificationPurposeEnum,
};
use palform_tsid::{
    resources::{IDAdminUser, IDAdminUserEmailVerification},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseTransaction, DbErr, EntityTrait,
    QueryFilter, QuerySelect, Set,
};
use serde::Serialize;
use thiserror::Error;

use crate::mail::{
    client::PalformMailClient,
    headers::{MailgunHeader, MailgunTemplateNameHeader, MailgunVariableListHeader},
};

pub struct EmailVerificationManager;

#[derive(Debug, Error)]
pub enum EmailVerificationError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Mail: {0}")]
    MailTransport(#[from] lettre::transport::smtp::Error),
    #[error("Construct email message: {0}")]
    MailStructure(#[from] lettre::error::Error),
    #[error("Parse email: {0}")]
    MailAddress(#[from] lettre::address::AddressError),
    #[error("Process mail variables: {0}")]
    MailVariables(#[from] serde_json::Error),
}

impl EmailVerificationManager {
    pub async fn process_verification(
        conn: &DatabaseTransaction,
        verification_id: PalformDatabaseID<IDAdminUserEmailVerification>,
    ) -> Result<admin_user_email_verification::Model, DbErr> {
        let verification = AdminUserEmailVerification::find_by_id(verification_id.clone())
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Verification".to_string()))?;

        AdminUserEmailVerification::delete_by_id(verification_id)
            .exec(conn)
            .await?;
        Ok(verification)
    }

    pub async fn send_email_verification<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        email_address: Option<String>,
        purpose: AdminUserEmailVerificationPurposeEnum,
        mail: &PalformMailClient,
    ) -> Result<(), EmailVerificationError> {
        let expires_at = Utc::now() + Duration::minutes(15);
        let verification_id = PalformDatabaseID::<IDAdminUserEmailVerification>::random();
        let new_verification_model = admin_user_email_verification::ActiveModel {
            id: Set(verification_id.clone()),
            user_id: Set(user_id.clone()),
            purpose: Set(purpose.clone()),
            expires_at: Set(expires_at.naive_utc()),
            ..Default::default()
        };
        new_verification_model.insert(conn).await?;

        let user_email: String;

        if let Some(email_address) = email_address {
            user_email = email_address
        } else {
            user_email = AdminUser::find_by_id(user_id)
                .select_only()
                .column(admin_user::Column::Email)
                .into_tuple()
                .one(conn)
                .await?
                .ok_or(DbErr::RecordNotFound("User not found".to_string()))?;
        }

        #[derive(Serialize)]
        struct EmailVariables {
            verification_id: String,
        }
        let email_variables = EmailVariables {
            verification_id: verification_id.to_string(),
        };
        let email_variables = serde_json::to_string(&email_variables)?;

        let b = mail.get_email_builder();
        let to_address: Mailbox = user_email.parse()?;
        let message = b
            .to(to_address)
            .subject("Verify your Palform email")
            .header(MailgunHeader::<MailgunTemplateNameHeader>::new(
                match purpose {
                    AdminUserEmailVerificationPurposeEnum::NewEmail => "verify_email",
                    AdminUserEmailVerificationPurposeEnum::PasswordReset => "password_reset",
                }
                .to_string(),
            ))
            .header(MailgunHeader::<MailgunVariableListHeader>::new(
                email_variables,
            ))
            .header(ContentType::TEXT_HTML)
            .body(Vec::new())?;

        mail.send_email(message).await?;
        Ok(())
    }

    pub async fn delete_abandoned_verifications<T: ConnectionTrait>(conn: &T) -> Result<(), DbErr> {
        AdminUserEmailVerification::delete_many()
            .filter(admin_user_email_verification::Column::ExpiresAt.lt(Utc::now()))
            .exec(conn)
            .await?;
        Ok(())
    }
}
