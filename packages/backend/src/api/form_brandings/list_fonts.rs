use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use rocket::{get, serde::json::Json};
use rocket_okapi::openapi;

use crate::auth::tokens::{APIAuthToken, APIAuthTokenSourcePersonal};

#[openapi(tag = "Form Brandings", operation_id = "google_fonts")]
#[get("/google-fonts")]
pub async fn handler(
    _token: APIAuthToken<APIAuthTokenSourcePersonal>,
) -> Result<Json<Vec<String>>, APIErrorWithStatus> {
    let fonts_string = include_str!("../../static/google_fonts.json");
    let family_names = serde_json::from_str(fonts_string)
        .map_err(|e| APIError::report_internal_error("parse fonts json", e))?;
    Ok(Json(family_names))
}
