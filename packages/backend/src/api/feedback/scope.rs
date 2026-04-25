use apistos::web::{post, resource, scope, Scope};

use crate::api::feedback::create::feedback_create;

pub fn feedback_scope() -> Scope {
    scope("feedback").service(resource("").route(post().to(feedback_create)))
}
