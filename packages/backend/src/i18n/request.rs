use palform_client_common::errors::error::APIError;
use rocket::{
    request::{self, FromRequest},
    serde::json::Json,
};
use rocket_okapi::{
    okapi::{openapi3::Parameter, schemars::schema::InstanceType, Map},
    request::OpenApiFromRequest,
};
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for I18NManager {
    type Error = Json<APIError>;
    async fn from_request(req: &'r request::Request<'_>) -> request::Outcome<Self, Self::Error> {
        let raw_header = req.headers().get_one("Accept-Language").unwrap_or("en");
        let supported_locales = available_locales!();
        let accepted_locales = accept_language::intersection(raw_header, &supported_locales);

        let chosen_locale = accepted_locales.first().cloned().unwrap_or("en".to_owned());
        request::Outcome::Success(I18NManager {
            locale: chosen_locale,
        })
    }
}

#[rocket::async_trait]
impl<'a> OpenApiFromRequest<'a> for I18NManager {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::Parameter(
            Parameter {
                name: "Accept-Language".to_owned(),
                required: false,
                description: Some("Defaults to `en`".to_owned()),
                location: "header".to_owned(),
                allow_empty_value: false,
                deprecated: false,
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
