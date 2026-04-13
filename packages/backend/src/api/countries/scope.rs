use apistos::web::{get, resource, scope, Scope};

use crate::api::countries::{
    list_calling_codes::countries_list_calling_codes, list_names::countries_list_names,
};

pub fn countries_scope() -> Scope {
    scope("countries")
        .service(resource("calling_codes").route(get().to(countries_list_calling_codes)))
        .service(resource("names").route(get().to(countries_list_names)))
}
