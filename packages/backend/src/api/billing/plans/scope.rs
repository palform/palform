use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::billing::plans::{
    cancel::billing_plans_cancel, get::billing_plans_get, initiate::billing_plans_initiate,
    list::billing_plans_list, switch::billing_plans_switch,
};

pub fn billing_public_plans_scope() -> Scope {
    scope("plans").service(resource("").route(get().to(billing_plans_list)))
}

pub fn billing_org_plans_scope() -> Scope {
    scope("plan")
        .service(resource("").route(get().to(billing_plans_get)))
        .service(resource("initiate").route(post().to(billing_plans_initiate)))
        .service(resource("switch").route(post().to(billing_plans_switch)))
        .service(resource("{stripe_subscription_id}").route(delete().to(billing_plans_cancel)))
}
