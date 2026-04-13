use std::{future::Future, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use apistos::ApiComponent;
use palform_client_common::errors::error::APIError;
use rust_i18n::available_locales;

pub struct I18NManager {
    locale: String,
}

impl I18NManager {
    pub fn get_locale(&self) -> &str {
        self.locale.as_str()
    }
}

#[macro_export]
macro_rules! pt {
    ($manager: ident, $key: literal, $($all_tokens:tt),*) => {
        rust_i18n::t!($key, locale = $manager.get_locale(), $($all_tokens)*).to_string()
    };
}

impl FromRequest for I18NManager {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let headers = req.headers().clone();

        Box::pin(async move {
            let raw_header = headers
                .get("Accept-Language")
                .map_or(Ok("en"), |v| v.to_str())
                .map_err(|_| {
                    APIError::ValidationError("Invalid language header value".to_string())
                })?;

            let supported_locales = available_locales!();
            let accepted_locales = accept_language::intersection(raw_header, &supported_locales);

            let chosen_locale = accepted_locales.first().cloned().unwrap_or("en".to_owned());
            Ok(I18NManager {
                locale: chosen_locale,
            })
        })
    }
}

/// Since `Accept-Language` is a default HTTP header, we don't really need to include this in the
/// OpenAPi schema.
impl ApiComponent for I18NManager {
    fn child_schemas() -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        Vec::default()
    }

    fn schema() -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        None
    }
}
