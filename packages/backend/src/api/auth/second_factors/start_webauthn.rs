use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{
    resources::IDAdminUserSecondAuthenticationFactorSession, tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use webauthn_rs::prelude::CreationChallengeResponse;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[derive(Serialize, JsonSchema)]
pub struct StartWebauthnResponse {
    #[schemars(with = "String")]
    pub ccr: CreationChallengeResponse,
    pub session: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
}

#[openapi(
    tag = "2FA Methods",
    operation_id = "user.second_factors.start_webauthn"
)]
#[post("/users/me/second_factors/webauthn/start")]
pub async fn handler(
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<StartWebauthnResponse>, APIErrorWithStatus> {
    let (ccr, id) = AdminUserSecondFactorManager::new(token.get_user_id(), config)
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .start_webauthn_register(db.inner())
        .await
        .map_err(|e| APIError::report_internal_error("start webauthn register", e))?;

    Ok(Json(StartWebauthnResponse { ccr, session: id }))
}
