use apistos::web::{patch, resource, scope, Scope};

use crate::api::admin_users::update::admin_users_update;

pub fn admin_users_scope() -> Scope {
    scope("/users").service(resource("/me").route(patch().to(admin_users_update)))
}
