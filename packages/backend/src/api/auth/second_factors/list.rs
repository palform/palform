use actix_web::web::{Data, Json};
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::admin_user_second_factor::APIAdminUserSecondAuthenticationFactor,
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[api_operation(tag = "2FA Methods", operation_id = "user.second_factors.list")]
pub async fn auth_second_factors_list(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
) -> Result<Json<Vec<APIAdminUserSecondAuthenticationFactor>>, APIError> {
    let resp = AdminUserSecondFactorManager::new(token.get_user_id(), config.as_ref())
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .list(db.as_ref())
        .await
        .map_internal_error()?;

    Ok(Json(resp))
}
