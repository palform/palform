use keshvar::CountryIterator;
use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use serde::Serialize;

#[derive(JsonSchema, Serialize)]
pub struct APICountryWithISOCode {
    name: String,
    iso_code: String,
}

#[openapi(tag = "Country Metadata", operation_id = "countries.list_names")]
#[get("/countries/names")]
pub fn handler() -> Json<Vec<APICountryWithISOCode>> {
    let mut list = Vec::<APICountryWithISOCode>::new();
    for country in CountryIterator::new() {
        list.push(APICountryWithISOCode {
            name: country.iso_short_name().to_string(),
            iso_code: country.alpha2().to_string(),
        });
    }

    Json(list)
}
