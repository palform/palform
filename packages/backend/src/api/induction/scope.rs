use apistos::web::{get, resource, scope, Scope};

use crate::api::induction::{alert::induction_alert, status::induction_status};

pub fn induction_scope() -> Scope {
    scope("induction")
        .service(resource("").route(get().to(induction_status)))
        .service(resource("alert").route(get().to(induction_alert)))
}
