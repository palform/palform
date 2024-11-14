use std::ops::Deref;

use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::{IDAdminUserSecondAuthenticationFactorSession, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::futures::TryFutureExt;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::auth::tokens::{NewAPIAuthToken, TokenManager};
use crate::captcha::VerifiedCaptcha;
use crate::entity_managers::admin_user_second_factors::{
    AdminUserSecondFactorManager, VerifyTOTPError,
};
use crate::entity_managers::admin_users::{AdminUserManagementError, AdminUserManager};
use crate::entity_managers::orgs::OrganisationManager;

#[derive(Deserialize, JsonSchema)]
pub enum SignInRequest {
    Credentials {
        email: String,
        password: String,
        create_initial_org: bool,
    },
    SecondFactor {
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        token: String,
    },
}

#[derive(Serialize, JsonSchema)]
pub enum SignInResponse {
    Done {
        token: NewAPIAuthToken,
        new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
    },
    SecondFactorRequired {
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
    },
}

#[openapi(tag = "Authentication", operation_id = "auth.sign_in")]
#[post("/auth/signin", data = "<request>")]
pub async fn handler(
    request: Json<SignInRequest>,
    db: &State<DatabaseConnection>,
    _captcha: VerifiedCaptcha,
    stripe: &State<stripe::Client>,
) -> Result<Json<SignInResponse>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let resp = match request.deref() {
        SignInRequest::Credentials {
            email,
            password,
            create_initial_org,
        } => {
            let user = AdminUserManager::get_user_by_email(&txn, email.to_owned())
                .await
                .map_internal_error()?
                .ok_or(APIError::BadRequest(
                    "Email or password incorrect".to_string(),
                ))?;

            let password_is_valid = AdminUserManager::verify_user_password(&user, password.to_owned())
        .map_err(|e|  match e {
                AdminUserManagementError::NoPassword => APIError::BadRequest(
                    "Your account belongs to an organisation. Please sign in via your organisation page instead."
                        .to_string()
                ).into(),
                    AdminUserManagementError::OrgOnly => APIError::BadRequest(e.to_string()).into(),
                _ => APIError::report_internal_error("verify password", e)
                })?;

            if !password_is_valid {
                return Err(APIError::BadRequest("Email or password incorrect".to_string()).into());
            }

            if !user.manual_auth_email_verified.is_some_and(|v| v) {
                return Err(APIError::BadRequest("Please verify your email address. If you didn't get an email, please contact hey@palform.app".to_string()).into());
            }

            let mut org_id = None::<PalformDatabaseID<IDOrganisation>>;
            if create_initial_org.to_owned() {
                let new_org_id =
                    OrganisationManager::create(&txn, "Unnamed organisation".to_string())
                        .await
                        .map_internal_error()?;
                org_id = Some(new_org_id);

                OrganisationManager::bootstrap_new_org(
                    &txn,
                    new_org_id,
                    user.id,
                    #[cfg(feature = "saas")]
                    stripe,
                )
                .await
                .map_err(|e| APIError::report_internal_error("bootstrap error", e))?;
            }

            let tfa_manager = AdminUserSecondFactorManager::new(user.id);
            if tfa_manager
                .user_requires_2fa(&txn)
                .await
                .map_internal_error()?
            {
                let session_id = tfa_manager
                    .create_auth_session(&txn)
                    .await
                    .map_internal_error()?;

                SignInResponse::SecondFactorRequired {
                    session_id,
                    new_org_id: org_id,
                }
            } else {
                let token = TokenManager::issue_token(&txn, user.id)
                    .map_err(|e| APIError::report_internal_error("issue token to password user", e))
                    .await?;
                SignInResponse::Done {
                    token,
                    new_org_id: org_id,
                }
            }
        }
        SignInRequest::SecondFactor { session_id, token } => {
            let (is_valid, user_id) = AdminUserSecondFactorManager::verify_auth_session(
                &txn,
                session_id.to_owned(),
                token.to_owned(),
            )
            .await
            .map_err(|e| match e {
                VerifyTOTPError::SessionExpired => APIError::BadRequest(e.to_string()).into(),
                VerifyTOTPError::SessionNotFound => APIError::BadRequest(e.to_string()).into(),
                _ => APIError::report_internal_error("verifying totp", e),
            })?;

            if is_valid {
                AdminUserSecondFactorManager::delete_auth_session(&txn, session_id.to_owned())
                    .await
                    .map_internal_error()?;

                let token = TokenManager::issue_token(&txn, user_id)
                    .await
                    .map_err(|e| APIError::report_internal_error("issue token following 2FA", e))?;

                SignInResponse::Done {
                    token,
                    new_org_id: None,
                }
            } else {
                return Err(APIError::BadRequest("Invalid 2FA token".to_string()).into());
            }
        }
    };

    txn.commit().await.map_internal_error()?;
    Ok(Json(resp))
}
