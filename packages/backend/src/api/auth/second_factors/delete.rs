use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDAdminUserSecondAuthenticationFactor, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[openapi(tag = "2FA Methods", operation_id = "user.second_factors.delete")]
#[delete("/users/me/second_factors/<factor_id>")]
pub async fn handler(
    factor_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    AdminUserSecondFactorManager::new(token.get_user_id())
        .delete(db.inner(), factor_id)
        .await
        .map_internal_error()?;
    Ok(())
}
