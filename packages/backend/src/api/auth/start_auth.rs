use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::auth::oidc::OIDCManager;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct StartAuthResponse {
    url: String,
    /// Make sure you verify the state returned in the callback is identical to this state value
    state: String,
    nonce: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct StartAuthQuery {
    redirect_url: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct StartAuthPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

/// Start authentication flow
///
/// Generate an OIDC URL with the configured provider to start authentication
#[api_operation(tag = "Authentication", operation_id = "auth.start")]
pub async fn auth_start_auth(
    path: Path<StartAuthPath>,
    Query(query): Query<StartAuthQuery>,
    db: Data<DatabaseConnection>,
) -> Result<Json<StartAuthResponse>, APIError> {
    let client = OIDCManager::get_client_for_org(db.as_ref(), path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("get org OIDC client", e))?;

    let (url, csrf, nonce) = client
        .authorization_url(&query.redirect_url)
        .map_err(|e| APIError::BadRequest(e.to_string()))?;

    Ok(Json(StartAuthResponse {
        url: url.to_string(),
        state: csrf.secret().to_owned(),
        nonce: nonce.secret().to_owned(),
    }))
}
