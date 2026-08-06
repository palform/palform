use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use apistos::ApiSecurity;
use base64::{prelude::BASE64_URL_SAFE, Engine};
use http::HeaderName;
use palform_client_common::errors::error::APIError;

use crate::{captcha::manager::CaptchaManager, config::Config, memory_db::memory_db::MemoryDB};

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(
    name = "X-Captcha-Response",
    api_key_in = "header"
))))]
pub struct VerifiedCaptcha;

pub static CAPTCHA_HEADER: HeaderName = HeaderName::from_static("x-captcha-response");

impl FromRequest for VerifiedCaptcha {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        let captcha_header_name = CAPTCHA_HEADER.clone();
        Box::pin(async move {
            let headers = req.headers();

            let captcha_response = headers
                .get(captcha_header_name)
                .ok_or(APIError::CaptchaError("Header not found".to_string()))?
                .to_str()
                .map_err(|e| APIError::CaptchaError(e.to_string()))?;
            let captcha_response = BASE64_URL_SAFE.decode(captcha_response).map_err(|e| {
                APIError::CaptchaError(format!("Parse solution base64: {}", e.to_string()))
            })?;
            let captcha_response: altcha::Payload = serde_json::from_slice(&captcha_response)
                .map_err(|e| {
                    APIError::CaptchaError(format!("Parse solution JSON: {}", e.to_string()))
                })?;

            let memory_db = req
                .app_data::<Data<MemoryDB>>()
                .ok_or_else(|| {
                    APIError::report_internal_error_without_error("MemoryDB not in state")
                })?
                .as_ref();
            let config = req
                .app_data::<Data<Config>>()
                .ok_or_else(|| {
                    APIError::report_internal_error_without_error("Config not in state")
                })?
                .as_ref();

            CaptchaManager::validate_challenge(captcha_response, &memory_db, &config)
                .await
                .map_err(|e| APIError::CaptchaError(e.to_string()))?;

            Ok(VerifiedCaptcha)
        })
    }
}
