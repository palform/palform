use sea_orm::{DatabaseConnection, DbErr};

use crate::entity_managers::email_verifications::EmailVerificationManager;

pub async fn job_delete_abandoned_emails(db: &DatabaseConnection) -> Result<(), DbErr> {
    EmailVerificationManager::delete_abandoned_verifications(db).await
}
