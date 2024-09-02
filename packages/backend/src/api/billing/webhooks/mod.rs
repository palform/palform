use crate::into_outcome;
use rocket::{
    http::Status,
    request::{self, FromRequest},
};

pub mod receiver;

pub struct StripeSignature(String);

#[rocket::async_trait]
impl<'a> FromRequest<'a> for StripeSignature {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let header = into_outcome!(request
            .headers()
            .get_one("Stripe-Signature")
            .ok_or((Status::BadRequest, "Missing signature header".to_string())));

        request::Outcome::Success(StripeSignature(header.to_string()))
    }
}
