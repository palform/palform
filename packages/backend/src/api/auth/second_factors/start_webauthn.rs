use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{
    resources::IDAdminUserSecondAuthenticationFactorSession, tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use webauthn_rs::prelude::CreationChallengeResponse;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct StartWebauthnResponse {
    #[schemars(with = "String")]
    pub ccr: CreationChallengeResponse,
    pub session: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
}

#[api_operation(tag = "2FA Methods", operation_id = "user.second_factors.start_webauthn")]
pub async fn auth_second_factors_start_webauthn(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
) -> Result<Json<StartWebauthnResponse>, APIError> {
    let (ccr, id) = AdminUserSecondFactorManager::new(token.get_user_id(), config.as_ref())
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .start_webauthn_register(db.as_ref())
        .await
        .map_err(|e| APIError::report_internal_error("start webauthn register", e))?;

    Ok(Json(StartWebauthnResponse { ccr, session: id }))
}
