use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, State};
use sea_orm::DatabaseConnection;

use crate::entity_managers::audit::AuditEntityManager;

#[post("/jobs/delete-old-audit-logs")]
pub async fn handler(db: &State<DatabaseConnection>) -> Result<(), APIErrorWithStatus> {
    AuditEntityManager::delete_old_entries(db.inner())
        .await
        .map_internal_error()
}
