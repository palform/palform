use chrono::Utc;
use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::fill_token::APIExchangedShortLink, auth::fill_access::FillAccessTokenManager,
    i18n::request::I18NManager, pt,
};

#[openapi(tag = "Forms", operation_id = "forms.exchange_short_link")]
#[get("/fill/short_link/<subdomain>/<short_link>")]
pub async fn handler(
    subdomain: &str,
    short_link: &str,
    db: &State<DatabaseConnection>,
    i18n: I18NManager,
) -> Result<Json<APIExchangedShortLink>, APIErrorWithStatus> {
    let resp = FillAccessTokenManager::get_short_link(
        db.inner(),
        subdomain.to_string(),
        short_link.to_string(),
    )
    .await
    .map_internal_error()?
    .ok_or(APIError::NotFound)?;

    if let Some(expires_at) = resp.expires_at {
        if expires_at < Utc::now().naive_utc() {
            return Err(APIError::BadRequest(pt!(i18n, "fill_form_expired",)).into());
        }
    }

    Ok(Json(resp))
}
