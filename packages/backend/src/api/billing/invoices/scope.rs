use apistos::web::{get, resource, scope, Scope};

use crate::api::billing::invoices::{
    list::billing_invoices_list, preview::billing_invoices_preview,
};

pub fn billing_invoices_scope() -> Scope {
    scope("invoices")
        .service(resource("").route(get().to(billing_invoices_list)))
        .service(resource("next").route(get().to(billing_invoices_preview)))
}
