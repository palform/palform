use std::{fmt::Display, future::Future, net::IpAddr, pin::Pin, str::FromStr};

use actix_web::{
    dev::Payload,
    web::{self, Data},
    FromRequest, HttpRequest,
};
use apistos::ApiComponent;
use palform_client_common::errors::error::APIError;
use schemars::JsonSchema;
use serde::Deserialize;
use stripe::Currency;

use crate::geo::IPGeolocator;

#[derive(Deserialize, JsonSchema, ApiComponent)]
struct ClientCurrencyQueryParams {
    currency: Option<String>,
}

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

impl FromRequest for ClientCurrency {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let param_fut = web::Query::<ClientCurrencyQueryParams>::from_request(req, payload);
        let req = req.clone();

        Box::pin(async move {
            let param = param_fut
                .await
                .map_err(|e| {
                    APIError::BadRequest(format!("Failed to parse params: {}", e.to_string()))
                })?
                .currency
                .clone();

            let currency_string = if let Some(param) = param {
                param
            } else {
                let connection_info = req.connection_info().to_owned();
                let client_ip = connection_info.realip_remote_addr();

                if let Some(client_ip) = client_ip {
                    let ip_geolocator = req.app_data::<Data<IPGeolocator>>().ok_or_else(|| {
                        APIError::report_internal_error_without_error(
                            "Missing IPGeolocator in state",
                        )
                    })?;

                    let client_ip = IpAddr::from_str(client_ip).map_err(|e| {
                        APIError::BadRequest(format!("Failed to parse client IP: {}", e))
                    })?;
                    let country = ip_geolocator.lookup_country(client_ip);

                    if let Ok(country) = country {
                        country.currency_code().to_string()
                    } else {
                        return Ok(ClientCurrency::default());
                    }
                } else {
                    return Ok(ClientCurrency::default());
                }
            };

            let currency = Currency::from_str(&currency_string)
                .map_err(|e| APIError::BadRequest(format!("Invalid currency: {}", e)))?;

            Ok(ClientCurrency(currency))
        })
    }
}

impl ApiComponent for ClientCurrency {
    fn child_schemas() -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        web::Query::<ClientCurrencyQueryParams>::child_schemas()
    }
    fn schema() -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        web::Query::<ClientCurrencyQueryParams>::schema()
    }
    fn raw_schema() -> Option<apistos::reference_or::ReferenceOr<apistos::Schema>> {
        web::Query::<ClientCurrencyQueryParams>::raw_schema()
    }
    fn parameters() -> Vec<apistos::paths::Parameter> {
        web::Query::<ClientCurrencyQueryParams>::parameters()
    }
}
