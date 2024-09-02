use palform_client_common::errors::error::APIError;
use rocket::{catch, serde::json::Json, Request};

#[catch(default)]
pub fn default_catcher<'a>(req: &'a Request) -> Json<APIError> {
    let err = req.local_cache(|| APIError::BadRequest("unknown".to_string()));
    Json(err.to_owned())
}
