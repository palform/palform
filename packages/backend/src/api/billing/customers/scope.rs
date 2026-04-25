use apistos::web::{get, post, resource, scope, Scope};

use crate::api::billing::customers::{
    get::billing_customers_get, update_payment_method::billing_customers_update_payment_method,
};

pub fn billing_customers_scope() -> Scope {
    scope("customer")
        .service(resource("").route(get().to(billing_customers_get)))
        .service(
            resource("payment_method_update_link")
                .route(post().to(billing_customers_update_payment_method)),
        )
}
