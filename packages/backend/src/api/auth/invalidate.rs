use actix_web::web::Data;
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use sea_orm::DatabaseConnection;

use crate::auth::tokens::{APIAuthToken, APIAuthTokenSourcePersonal, TokenManager};

#[api_operation(tag = "Authentication", operation_id = "auth.invalidate_token")]
pub async fn auth_invalidate(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    TokenManager::delete_token_by_id(db.as_ref(), token.model.id)
        .await
        .map_internal_error()?;
    Ok(())
}
