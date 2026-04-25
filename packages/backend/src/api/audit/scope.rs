use apistos::web::{get, resource, scope, Scope};

use crate::api::audit::list::audit_logs_list;

pub fn audit_logs_scope() -> Scope {
    scope("audit").service(resource("").route(get().to(audit_logs_list)))
}
