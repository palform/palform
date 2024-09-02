use keshvar::CountryIterator;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(JsonSchema, Serialize)]
pub struct APICountryWithCallingCode {
    flag_emoji: String,
    name: String,
    calling_code: usize,
}

#[openapi(
    tag = "Country Metadata",
    operation_id = "countries.list_calling_codes"
)]
#[get("/countries/calling_codes")]
pub fn handler() -> Json<Vec<APICountryWithCallingCode>> {
    let mut list = Vec::<APICountryWithCallingCode>::new();
    for country in CountryIterator::new() {
        list.push(APICountryWithCallingCode {
            flag_emoji: country.emoji().to_string(),
            name: country.iso_short_name().to_string(),
            calling_code: country.country_code(),
        });
    }

    Json(list)
}
