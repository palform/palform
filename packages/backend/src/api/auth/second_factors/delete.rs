use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDAdminUserSecondAuthenticationFactor, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SecondFactorsDeletePath {
    factor_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>,
}

#[api_operation(tag = "2FA Methods", operation_id = "user.second_factors.delete")]
pub async fn auth_second_factors_delete(
    path: Path<SecondFactorsDeletePath>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
) -> Result<(), APIError> {
    AdminUserSecondFactorManager::new(token.get_user_id(), config.as_ref())
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .delete(db.as_ref(), path.factor_id)
        .await
        .map_internal_error()?;
    Ok(())
}
