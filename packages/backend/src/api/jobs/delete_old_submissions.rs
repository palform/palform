use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, State};
use sea_orm::DatabaseConnection;

use crate::entity_managers::forms::FormManager;

#[post("/jobs/delete-old-submissions")]
pub async fn handler(db: &State<DatabaseConnection>) -> Result<(), APIErrorWithStatus> {
    FormManager::delete_all_old_submissions(db.inner())
        .await
        .map_internal_error()
}
