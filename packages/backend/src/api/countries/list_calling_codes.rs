use actix_web::web::Json;
use apistos::{api_operation, ApiComponent};
use keshvar::CountryIterator;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, ApiComponent)]
pub struct APICountryWithCallingCode {
    flag_emoji: String,
    name: String,
    calling_code: usize,
}

#[api_operation(tag = "Country Metadata", operation_id = "countries.list_calling_codes")]
pub async fn countries_list_calling_codes() -> Json<Vec<APICountryWithCallingCode>> {
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
