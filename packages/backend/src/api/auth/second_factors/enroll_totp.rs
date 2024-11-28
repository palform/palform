use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDAdminUserSecondAuthenticationFactor;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::post;
use rocket::{serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::config::Config;
use crate::entity_managers::admin_user_second_factors::AdminUserSecondFactorManager;
use crate::rocket_util::validated::Validated;

#[derive(Deserialize, JsonSchema, Validate)]
pub struct EnrollTOTPRequest {
    secret: String,
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    nickname: String,
}

#[openapi(tag = "2FA Methods", operation_id = "user.second_factors.enroll")]
#[post("/users/me/second_factors/totp", data = "<data>")]
pub async fn handler(
    data: Validated<Json<EnrollTOTPRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>>, APIErrorWithStatus> {
    let id = AdminUserSecondFactorManager::new(token.get_user_id(), config)
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .register_totp(db.inner(), data.nickname.clone(), data.secret.clone())
        .await
        .map_internal_error()?;
    Ok(Json(id))
}
