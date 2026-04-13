use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{
    resources::{
        IDAdminUserSecondAuthenticationFactor, IDAdminUserSecondAuthenticationFactorSession,
    },
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use webauthn_rs::prelude::RegisterPublicKeyCredential;

use crate::{
    auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal},
    config::Config,
    entity_managers::admin_user_second_factors::AdminUserSecondFactorManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct EnrollWebauthnRequest {
    #[schemars(with = "String")]
    cred: RegisterPublicKeyCredential,
    session: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
    nickname: String,
}

#[api_operation(tag = "2FA Methods", operation_id = "user.second_factors.enroll_webauthn")]
pub async fn auth_second_factors_enroll_webauthn(
    data: Json<EnrollWebauthnRequest>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
) -> Result<Json<PalformDatabaseID<IDAdminUserSecondAuthenticationFactor>>, APIError> {
    let id = AdminUserSecondFactorManager::new(token.get_user_id(), config.as_ref())
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?
        .register_webauthn(
            db.as_ref(),
            data.nickname.clone(),
            data.session,
            data.cred.clone(),
        )
        .await
        .map_err(|e| APIError::report_internal_error("start webauthn register", e))?;

    Ok(Json(id))
}
