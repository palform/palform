use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::api_entities::admin_users::APIAdminUser;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::entity_managers::admin_users::AdminUserManager;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct AuthTestResponse {
    token_id: String,
    user: APIAdminUser,
}

/// Test authentication
///
/// Checks whether the provided Authorization header is valid and returns the user's ID
#[api_operation(tag = "Authentication", operation_id = "auth.test")]
pub async fn auth_test(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
) -> Result<Json<AuthTestResponse>, APIError> {
    let user = AdminUserManager::get_user_by_id(db.as_ref(), token.get_user_id())
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::Internal)?;

    Ok(Json(AuthTestResponse {
        token_id: token.model.id.to_string(),
        user,
    }))
}
