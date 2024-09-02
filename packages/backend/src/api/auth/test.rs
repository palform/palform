use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalError};
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::api_entities::admin_users::APIAdminUser;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::entity_managers::admin_users::AdminUserManager;

#[derive(Serialize, JsonSchema)]
pub struct AuthTestResponse {
    token_id: String,
    user: APIAdminUser,
}

/// Test authentication
///
/// Checks whether the provided Authorization header is valid and returns the user's ID
#[openapi(tag = "Authentication", operation_id = "auth.test")]
#[get("/auth/test")]
pub async fn handler(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<Json<AuthTestResponse>, APIErrorWithStatus> {
    let user = AdminUserManager::get_user_by_id(db.inner(), token.get_user_id())
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::Internal)?;

    Ok(Json(AuthTestResponse {
        token_id: token.model.id.to_string(),
        user,
    }))
}
