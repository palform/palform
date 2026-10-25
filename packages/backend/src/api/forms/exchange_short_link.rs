use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use chrono::Utc;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::fill_token::APIExchangedShortLink, auth::fill_access::FillAccessTokenManager,
    i18n::request::I18NManager, pt,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsExchangeShortLinkPath {
    /// The organisation's subdomain
    subdomain: String,
    /// The form's short link path
    short_link: String,
}

/// Resolve a short link on an organisational subdomain to an organisation ID, a form ID, and a fill
/// token.
#[api_operation(tag = "Forms", operation_id = "forms.exchange_short_link", no_security)]
pub async fn forms_exchange_short_link(
    path: Path<FormsExchangeShortLinkPath>,
    db: Data<DatabaseConnection>,
    i18n: I18NManager,
) -> Result<Json<APIExchangedShortLink>, APIError> {
    let resp = FillAccessTokenManager::get_short_link(
        db.as_ref(),
        path.subdomain.clone(),
        path.short_link.clone(),
    )
    .await
    .map_internal_error()?
    .ok_or(APIError::NotFound)?;

    if let Some(expires_at) = resp.expires_at {
        if expires_at < Utc::now() {
            return Err(APIError::BadRequest(pt!(i18n, "fill_form_expired",)).into());
        }
    }

    Ok(Json(resp))
}
