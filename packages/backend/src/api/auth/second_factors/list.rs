use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::admin_user_second_factor::APIAdminUserSecondAuthenticationFactor,
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[openapi(tag = "2FA Methods", operation_id = "user.second_factors.list")]
#[get("/users/me/second_factors")]
pub async fn handler(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIAdminUserSecondAuthenticationFactor>>, APIErrorWithStatus> {
    let resp = AdminUserSecondFactorManager::new(token.get_user_id())
        .list(db.inner())
        .await
        .map_internal_error()?;

    Ok(Json(resp))
}
