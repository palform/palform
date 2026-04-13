use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use apistos::ApiSecurity;
use http::HeaderName;
use palform_client_common::errors::error::APIError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::config::Config;

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(
    name = "X-Captcha-Response",
    api_key_in = "header"
))))]
pub struct VerifiedCaptcha;

#[derive(Debug, Error)]
pub enum CaptchaVerificationError {
    #[error("connection: {0}")]
    Request(#[from] reqwest::Error),
    #[error("failed")]
    CaptchaFailed,
}

#[derive(Serialize)]
struct CaptchaVerificationRequest {
    secret: String,
    response: String,
}

#[derive(Deserialize)]
struct CaptchaVerificationResponse {
    success: bool,
}

impl VerifiedCaptcha {
    async fn verify_token(config: &Config, token: &str) -> Result<(), CaptchaVerificationError> {
        if config.skip_captcha {
            return Ok(());
        }

        let params = CaptchaVerificationRequest {
            secret: config.captcha_secret_key.clone(),
            response: token.to_owned(),
        };

        let resp = Client::default()
            .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
            .form(&params)
            .send()
            .await?
            .json::<CaptchaVerificationResponse>()
            .await?;

        if !resp.success {
            Err(CaptchaVerificationError::CaptchaFailed)
        } else {
            Ok(())
        }
    }
}

pub static CAPTCHA_HEADER: HeaderName = HeaderName::from_static("x-captcha-response");

impl FromRequest for VerifiedCaptcha {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        let captcha_header_name = CAPTCHA_HEADER.clone();
        Box::pin(async move {
            let header = req
                .headers()
                .get(captcha_header_name)
                .ok_or(APIError::CaptchaError("Header not found".to_string()))?
                .to_str()
                .map_err(|e| APIError::CaptchaError(e.to_string()))?;

            let config = req.app_data::<Data<Config>>().ok_or_else(|| {
                APIError::report_internal_error_without_error("Config not in state")
            })?;

            VerifiedCaptcha::verify_token(config, header)
                .await
                .map_err(|e| APIError::CaptchaError(e.to_string()))?;

            Ok(VerifiedCaptcha)
        })
    }
}
