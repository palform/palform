use palform_client_common::errors::error::APIError;
use rocket::{
    request::{self, FromRequest},
    serde::json::Json,
};
use rocket_okapi::{
    okapi::{schemars::schema::InstanceType, Map},
    request::OpenApiFromRequest,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{config::Config, into_outcome};

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

        let resp = reqwest::Client::default()
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

#[rocket::async_trait]
impl<'a> FromRequest<'a> for VerifiedCaptcha {
    type Error = Json<APIError>;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let header = into_outcome!(
            request
                .headers()
                .get_one("X-Captcha-Response")
                .ok_or(APIError::CaptchaError("Header not found".to_string())),
            request
        );

        let config = into_outcome!(request.rocket().state::<Config>().ok_or(APIError::Internal));

        into_outcome!(
            VerifiedCaptcha::verify_token(config, header)
                .await
                .map_err(|e| APIError::CaptchaError(e.to_string())),
            request
        );

        request::Outcome::Success(VerifiedCaptcha)
    }
}

#[rocket::async_trait]
impl<'a> OpenApiFromRequest<'a> for VerifiedCaptcha {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::Parameter(
            rocket_okapi::okapi::openapi3::Parameter {
                name: "X-Captcha-Response".to_string(),
                location: "header".to_string(),
                description: Some("Response from the Cloudflare Turnstile captcha".to_string()),
                required,
                deprecated: false,
                allow_empty_value: false,
                extensions: Map::new(),
                value: rocket_okapi::okapi::openapi3::ParameterValue::Schema {
                    style: Some(rocket_okapi::okapi::openapi3::ParameterStyle::Simple),
                    schema: rocket_okapi::okapi::openapi3::SchemaObject {
                        instance_type: Some(
                            rocket_okapi::okapi::schemars::schema::SingleOrVec::Single(Box::new(
                                InstanceType::String,
                            )),
                        ),
                        ..Default::default()
                    },
                    explode: None,
                    example: None,
                    examples: None,
                    allow_reserved: false,
                },
            },
        ))
    }
}
