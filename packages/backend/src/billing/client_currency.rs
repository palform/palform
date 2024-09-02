use std::str::FromStr;

use rocket::{
    http::Status,
    request::{self, FromRequest},
};
use rocket_okapi::{
    okapi::{
        openapi3::{ParameterValue, SchemaObject},
        schemars::{
            schema::{InstanceType, SingleOrVec},
            Map,
        },
    },
    request::OpenApiFromRequest,
};
use stripe::Currency;

use crate::into_outcome;

pub struct ClientCurrency(Currency);

impl From<ClientCurrency> for Currency {
    fn from(value: ClientCurrency) -> Self {
        value.0
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ClientCurrency {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let param = into_outcome!(request
            .query_value::<&str>("currency")
            .ok_or((Status::BadRequest, "Missing currency param".to_string()))
            .and_then(|e| e.map_err(|e| (Status::BadRequest, e.to_string()))));

        let currency = into_outcome!(
            Currency::from_str(param).map_err(|e| (Status::BadRequest, e.to_string()))
        );

        request::Outcome::Success(ClientCurrency(currency))
    }
}

impl<'a> OpenApiFromRequest<'a> for ClientCurrency {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::Parameter(
            rocket_okapi::okapi::openapi3::Parameter {
                name: "currency".to_string(),
                location: "query".to_string(),
                description: Some("The currency the client is using".to_string()),
                required: true,
                deprecated: false,
                allow_empty_value: false,
                extensions: Map::new(),
                value: ParameterValue::Schema {
                    style: None,
                    explode: None,
                    allow_reserved: false,
                    example: None,
                    examples: None,
                    schema: SchemaObject {
                        instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
                        ..Default::default()
                    },
                },
            },
        ))
    }
}
