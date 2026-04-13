use apistos::web::{get, post, resource, scope, Scope};

use crate::api::billing::entitlements::{
    list::billing_entitlements_list, test::billing_entitlements_test,
};

pub fn billing_entitlements_scope() -> Scope {
    scope("entitlements")
        .service(resource("").route(get().to(billing_entitlements_list)))
        .service(resource("test").route(post().to(billing_entitlements_test)))
}
