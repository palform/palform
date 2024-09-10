use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::serde::json::Json;
use rocket::{patch, State};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::entity_managers::admin_users::AdminUserManager;
use crate::rocket_util::validated::Validated;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct UpdateAdminUserRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 40 characters"))]
    pub display_name: Option<String>,
}

#[openapi(tag = "Admin Users", operation_id = "admin_users.update")]
#[patch("/users/me", data = "<data>")]
pub async fn handler(
    data: Validated<Json<UpdateAdminUserRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    AdminUserManager::update_user_profile(
        db.inner(),
        token.get_user_id(),
        data.display_name.to_owned(),
    )
    .await
    .map_internal_error()?;
    Ok(())
}
