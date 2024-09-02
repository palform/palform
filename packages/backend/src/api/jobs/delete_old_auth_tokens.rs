use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, State};
use sea_orm::DatabaseConnection;

use crate::auth::tokens::TokenManager;

#[post("/jobs/delete-old-auth-tokens")]
pub async fn handler(db: &State<DatabaseConnection>) -> Result<(), APIErrorWithStatus> {
    TokenManager::delete_all_old_tokens(db.inner())
        .await
        .map_internal_error()
}
