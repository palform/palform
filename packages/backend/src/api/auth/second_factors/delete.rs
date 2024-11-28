use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDAdminUserSecondAuthenticationFactor, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[openapi(tag = "2FA Methods", operation_id = "user.second_factors.delete")]
#[delete("/users/me/second_factors/<factor_id>")]
pub async fn handler(
    factor_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<(), APIErrorWithStatus> {
    AdminUserSecondFactorManager::new(token.get_user_id(), config)
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .delete(db.inner(), factor_id)
        .await
        .map_internal_error()?;
    Ok(())
}
