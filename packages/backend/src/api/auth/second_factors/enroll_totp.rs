use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::resources::IDAdminUserSecondAuthenticationFactor;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use validator::Validate;

use crate::actix_util::validated::Validated;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::config::Config;
use crate::entity_managers::admin_user_second_factors::AdminUserSecondFactorManager;

#[derive(Deserialize, JsonSchema, Validate, ApiComponent)]
pub struct EnrollTOTPRequest {
    secret: String,
    #[validate(length(min = 1, max = 30, message = "must be between 1 and 30 characters"))]
    nickname: String,
}

#[api_operation(tag = "2FA Methods", operation_id = "user.second_factors.enroll")]
pub async fn auth_second_factors_enroll_totp(
    data: Validated<Json<EnrollTOTPRequest>>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
) -> Result<Json<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>>, APIError> {
    let id = AdminUserSecondFactorManager::new(token.get_user_id(), config.as_ref())
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .register_totp(db.as_ref(), data.nickname.clone(), data.secret.clone())
        .await
        .map_internal_error()?;
    Ok(Json(id))
}
