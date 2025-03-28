use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::IDAdminUserSecondAuthenticationFactorSession, tsid::PalformDatabaseID,
};
use rocket::post;
use rocket::{serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;
use webauthn_rs::prelude::PublicKeyCredential;

use crate::{
    auth::tokens::TokenManager,
    config::Config,
    entity_managers::admin_user_second_factors::{
        AdminUserSecondFactorManager, ProcessWebauthnError, VerifyTOTPError,
    },
};

use super::common::SignInResponse;

#[derive(Deserialize, JsonSchema, Clone)]
pub enum VerifyTFASecondFactorRequest {
    Totp(String),
    Webauthn(#[schemars(with = "String")] PublicKeyCredential),
}

#[derive(Deserialize, JsonSchema)]
pub struct VerifyTFARequest {
    session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
    factor: VerifyTFASecondFactorRequest,
}

#[openapi(tag = "Authentication", operation_id = "auth.verify_tfa")]
#[post("/auth/tfa", data = "<request>")]
pub async fn handler(
    request: Json<VerifyTFARequest>,
    db: &State<DatabaseConnection>,
    config: &State<Config>,
) -> Result<Json<SignInResponse>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let (is_valid, user_id) = match request.factor.to_owned() {
        VerifyTFASecondFactorRequest::Totp(token) => {
            AdminUserSecondFactorManager::verify_totp_session(
                &txn,
                request.session_id.to_owned(),
                token.to_owned(),
            )
            .await
            .map_err(|e| match e {
                VerifyTOTPError::SessionExpired => APIError::BadRequest(e.to_string()).into(),
                VerifyTOTPError::SessionNotFound => APIError::BadRequest(e.to_string()).into(),
                _ => APIError::report_internal_error("verifying totp", e),
            })?
        }
        VerifyTFASecondFactorRequest::Webauthn(pkc) => {
            AdminUserSecondFactorManager::verify_webauthn_session(
                &txn,
                config,
                request.session_id.to_owned(),
                pkc.to_owned(),
            )
            .await
            .map_err(|e| match e {
                ProcessWebauthnError::Session(_) => APIError::BadRequest(e.to_string()).into(),
                _ => APIError::report_internal_error("verifying webauthn", e),
            })?
        }
    };

    let resp = if is_valid {
        AdminUserSecondFactorManager::delete_auth_session(&txn, request.session_id.to_owned())
            .await
            .map_internal_error()?;

        let token = TokenManager::issue_token(&txn, user_id)
            .await
            .map_err(|e| APIError::report_internal_error("issue token following 2FA", e))?;

        Ok(Json(SignInResponse::Done {
            token,
            new_org_id: None,
        }))
    } else {
        Err(APIError::BadRequest("Invalid 2FA token".to_string()).into())
    };

    txn.commit().await.map_internal_error()?;
    resp
}
