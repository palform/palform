use std::{future::Future, ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::Json, FromRequest, HttpRequest};
use apistos::ApiComponent;
use palform_client_common::errors::error::APIError;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct Validated<T>(pub T);

impl<T> Deref for Validated<Json<T>> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<D: Validate + DeserializeOwned + 'static> FromRequest for Validated<Json<D>> {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let data_fut = Json::<D>::from_request(req, payload);
        Box::pin(async move {
            let data = data_fut
                .await
                .map_err(|e| APIError::BadRequest(e.to_string()))?;

            data.validate()
                .map_err(|e| APIError::BadRequest(e.to_string()))?;

            Ok(Validated(data))
        })
    }
}

impl<T> ApiComponent for Validated<T>
where
    T: ApiComponent,
{
    fn child_schemas() -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        T::child_schemas()
    }
    fn raw_schema() -> Option<apistos::reference_or::ReferenceOr<apistos::Schema>> {
        T::raw_schema()
    }
    fn schema() -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        T::schema()
    }
}
