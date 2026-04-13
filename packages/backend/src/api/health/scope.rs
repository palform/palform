use apistos::web::{get, resource, scope, Scope};

use crate::api::health::ping::health_ping_handler;

pub fn health_scope() -> Scope {
    scope("health").service(resource("").route(get().to(health_ping_handler)))
}
