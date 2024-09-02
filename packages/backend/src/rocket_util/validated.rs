use std::ops::Deref;

use palform_client_common::errors::error::APIError;
use rocket::{data::FromData, http::Status, outcome::Outcome, serde::json::Json, Data, Request};
use rocket_okapi::{
    gen::OpenApiGenerator, okapi::openapi3::RequestBody, request::OpenApiFromData, JsonSchema,
    OpenApiError,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct Validated<T>(pub T);

impl<T> Deref for Validated<Json<T>> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[rocket::async_trait]
impl<'r, D: Validate + Deserialize<'r>> FromData<'r> for Validated<Json<D>> {
    type Error = Json<APIError>;
    async fn from_data(
        req: &'r Request<'_>,
        data: Data<'r>,
    ) -> Outcome<Self, (Status, Self::Error), (Data<'r>, Status)> {
        let data = <Json<D> as FromData<'r>>::from_data(req, data).await;
        match data {
            Outcome::Error((status, err)) => {
                Outcome::Error((status, Json(APIError::BadRequest(err.to_string()))))
            }
            Outcome::Forward(err) => Outcome::Forward(err),
            Outcome::Success(data) => match data.validate() {
                Ok(_) => Outcome::Success(Validated(data)),
                Err(err) => {
                    let err = APIError::ValidationError(err.to_string());
                    req.local_cache(|| err.clone());
                    Outcome::Error(err.into())
                }
            },
        }
    }
}

impl<'r, D: Validate + Deserialize<'r> + JsonSchema> OpenApiFromData<'r> for Validated<Json<D>> {
    fn request_body(gen: &mut OpenApiGenerator) -> Result<RequestBody, OpenApiError> {
        <Json<D>>::request_body(gen)
    }
}
