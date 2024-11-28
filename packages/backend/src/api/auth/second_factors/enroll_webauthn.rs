use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{
    resources::{
        IDAdminUserSecondAuthenticationFactor, IDAdminUserSecondAuthenticationFactorSession,
    },
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct EnrollWebauthnRequest {
    #[schemars(with = "String")]
    cred: RegisterPublicKeyCredential,
    session: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
    nickname: String,
}

#[openapi(
    tag = "2FA Methods",
    operation_id = "user.second_factors.enroll_webauthn"
)]
#[post("/users/me/second_factors/webauthn", data = "<data>")]
pub async fn handler(
    data: Json<EnrollWebauthnRequest>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>>, APIErrorWithStatus> {
    let id = AdminUserSecondFactorManager::new(token.get_user_id(), config)
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .register_webauthn(
            db.inner(),
            data.nickname.clone(),
            data.session,
            data.cred.clone(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("start webauthn register", e))?;

    Ok(Json(id))
}
