use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::auth::tokens::{APIAuthToken, APIAuthTokenSourcePersonal, TokenManager};

#[openapi(tag = "Authentication", operation_id = "auth.invalidate_token")]
#[delete("/users/me/tokens/current")]
pub async fn handler(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    TokenManager::delete_token_by_id(db.inner(), token.model.id)
        .await
        .map_internal_error()?;
    Ok(())
}
