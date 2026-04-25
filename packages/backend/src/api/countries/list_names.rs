use actix_web::web::Json;
use apistos::{api_operation, ApiComponent};
use keshvar::CountryIterator;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, ApiComponent)]
pub struct APICountryWithISOCode {
    name: String,
    iso_code: String,
}

#[api_operation(tag = "Country Metadata", operation_id = "countries.list_names")]
pub async fn countries_list_names() -> Json<Vec<APICountryWithISOCode>> {
    let mut list = Vec::<APICountryWithISOCode>::new();
    for country in CountryIterator::new() {
        list.push(APICountryWithISOCode {
            name: country.iso_short_name().to_string(),
            iso_code: country.alpha2().to_string(),
        });
    }

    Json(list)
}
