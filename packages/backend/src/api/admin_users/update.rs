use actix_web::web::{self};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::actix_util::validated::Validated;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::entity_managers::admin_users::AdminUserManager;

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct UpdateAdminUserRequest {
    #[validate(length(min = 1, max = 40, message = "must be between 1 and 40 characters"))]
    pub display_name: Option<String>,
}

#[api_operation(tag = "Admin Users", operation_id = "admin_users.update")]
pub async fn admin_users_update(
    data: Validated<web::Json<UpdateAdminUserRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: web::Data<DatabaseConnection>,
) -> Result<(), APIError> {
    AdminUserManager::update_user_profile(
        db.get_ref(),
        token.get_user_id(),
        data.display_name.to_owned(),
    )
    .await
    .map_internal_error()?;
    Ok(())
}
