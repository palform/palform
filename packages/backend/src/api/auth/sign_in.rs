use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::futures::TryFutureExt;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::auth::tokens::TokenManager;
use crate::captcha::VerifiedCaptcha;
use crate::config::Config;
use crate::entity_managers::admin_user_second_factors::AdminUserSecondFactorManager;
use crate::entity_managers::admin_users::{AdminUserManagementError, AdminUserManager};
use crate::entity_managers::orgs::OrganisationManager;

use super::common::SignInResponse;

#[derive(Deserialize, JsonSchema)]
pub struct SignInRequest {
    email: String,
    password: String,
    create_initial_org: bool,
}

#[openapi(tag = "Authentication", operation_id = "auth.sign_in")]
#[post("/auth/signin", data = "<request>")]
pub async fn handler(
    request: Json<SignInRequest>,
    db: &State<DatabaseConnection>,
    _captcha: VerifiedCaptcha,
    stripe: &State<stripe::Client>,
    config: &State<Config>,
) -> Result<Json<SignInResponse>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let user = AdminUserManager::get_user_by_email(&txn, request.email.to_owned())
        .await
        .map_internal_error()?
        .ok_or(APIError::BadRequest(
            "Email or password incorrect".to_string(),
        ))?;

    let password_is_valid = AdminUserManager::verify_user_password(&user, request.password.to_owned())
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
    if request.create_initial_org {
        let new_org_id = OrganisationManager::create(&txn, "Unnamed organisation".to_string())
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

    let tfa_manager = AdminUserSecondFactorManager::new(user.id, config)
        .map_err(|e| APIError::report_internal_error("init 2fa manager", e))?;

    let (requires_totp, requires_webauthn) = tfa_manager
        .user_requires_2fa(&txn)
        .await
        .map_internal_error()?;

    let resp = if requires_totp || requires_webauthn {
        let (session_id, rcr) = tfa_manager
            .create_auth_session(&txn)
            .await
            .map_err(|e| APIError::report_internal_error("init 2fa auth session", e))?;

        SignInResponse::SecondFactorRequired {
            session_id,
            rcr,
            totp: requires_totp,
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
    };

    txn.commit().await.map_internal_error()?;
    Ok(Json(resp))
}
