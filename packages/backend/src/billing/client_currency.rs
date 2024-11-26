use std::{fmt::Display, str::FromStr};

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

use crate::{geo::IPGeolocator, into_outcome};

#[derive(Clone)]
pub struct ClientCurrency(Currency);

impl From<ClientCurrency> for Currency {
    fn from(value: ClientCurrency) -> Self {
        value.0
    }
}

impl Default for ClientCurrency {
    fn default() -> Self {
        Self(Currency::GBP)
    }
}

impl Display for ClientCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for ClientCurrency {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let param = request.query_value::<&str>("currency");

        let currency_string = if let Some(param) = param {
            into_outcome!(param.map_err(|e| (Status::BadRequest, e.to_string()))).to_owned()
        } else {
            let client_ip = request.client_ip();

            if let Some(client_ip) = client_ip {
                let ip_geolocator =
                    into_outcome!(request.rocket().state::<IPGeolocator>().ok_or((
                        Status::InternalServerError,
                        "Missing IPGeolocator in state".to_string(),
                    )));

                let country = ip_geolocator.lookup_country(client_ip);

                if let Ok(country) = country {
                    country.currency_code().to_string()
                } else {
                    return request::Outcome::Success(ClientCurrency::default());
                }
            } else {
                return request::Outcome::Success(ClientCurrency::default());
            }
        };

        let currency =
            into_outcome!(Currency::from_str(&currency_string)
                .map_err(|e| (Status::BadRequest, e.to_string())));

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
                description: Some("The currency the client is using, or the default currency based on the client's IP address country (if not provided).".to_string()),
                required: false,
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
