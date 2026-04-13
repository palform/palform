use apistos::web::{scope, Scope};

use crate::api::billing::{
    customers::scope::billing_customers_scope,
    entitlements::scope::billing_entitlements_scope,
    invoices::scope::billing_invoices_scope,
    plans::scope::{billing_org_plans_scope, billing_public_plans_scope},
    webhooks::scope::billing_webhooks_scope,
};

pub fn billing_scope() -> Scope {
    scope("billing")
        .service(billing_customers_scope())
        .service(billing_entitlements_scope())
        .service(billing_invoices_scope())
        .service(billing_org_plans_scope())
}

pub fn billing_public_scope() -> Scope {
    scope("billing")
        .service(billing_public_plans_scope())
        .service(billing_webhooks_scope())
}
