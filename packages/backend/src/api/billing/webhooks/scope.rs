use apistos::web::{post, resource, scope, Scope};

use crate::api::billing::webhooks::receiver::billing_webhooks_receiver_handler;

pub fn billing_webhooks_scope() -> Scope {
    scope("webhook").service(resource("").route(post().to(billing_webhooks_receiver_handler)))
}
