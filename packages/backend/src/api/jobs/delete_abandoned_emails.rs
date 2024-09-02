use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, State};
use sea_orm::DatabaseConnection;

use crate::entity_managers::email_verifications::EmailVerificationManager;

#[post("/jobs/delete-abandoned-emails")]
pub async fn handler(db: &State<DatabaseConnection>) -> Result<(), APIErrorWithStatus> {
    EmailVerificationManager::delete_abandoned_verifications(db.inner())
        .await
        .map_internal_error()
}
