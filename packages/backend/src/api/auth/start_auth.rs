use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::{get, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::auth::oidc::OIDCManager;

#[derive(Serialize, JsonSchema)]
pub struct StartAuthResponse {
    url: String,
    /// Make sure you verify the state returned in the callback is identical to this state value
    state: String,
    nonce: String,
}

/// Start authentication flow
///
/// Generate an OIDC URL with the configured provider to start authentication
#[openapi(tag = "Authentication", operation_id = "auth.start")]
#[get("/auth/<org_id>/start?<redirect_url>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    redirect_url: &str,
    db: &State<DatabaseConnection>,
) -> Result<Json<StartAuthResponse>, APIErrorWithStatus> {
    let client = OIDCManager::get_client_for_org(db.inner(), org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get org OIDC client", e))?;

    let (url, csrf, nonce) = client
        .authorization_url(redirect_url)
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    Ok(Json(StartAuthResponse {
        url: url.to_string(),
        state: csrf.secret().to_owned(),
        nonce: nonce.secret().to_owned(),
    }))
}
