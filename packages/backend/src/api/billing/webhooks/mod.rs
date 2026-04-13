use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use apistos::ApiHeader;
use palform_client_common::errors::error::APIError;
use schemars::JsonSchema;

pub mod receiver;
pub mod scope;

#[derive(ApiHeader, JsonSchema)]
#[openapi_header(name = "Stripe-Signature", required = true)]
pub struct StripeSignature(String);

impl FromRequest for StripeSignature {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers().clone();
        Box::pin(async move {
            let header = headers
                .get("Stripe-Signature")
                .ok_or(APIError::BadRequest("Missing signature header".to_string()))?
                .to_str()
                .map_err(|e| APIError::BadRequest(e.to_string()))?;

            Ok(StripeSignature(header.to_owned()))
        })
    }
}
