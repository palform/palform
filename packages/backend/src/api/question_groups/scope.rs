use apistos::web::{get, resource, scope, Scope};

use crate::api::question_groups::list::question_groups_list;

pub fn question_groups_scope() -> Scope {
    scope("groups").service(resource("").route(get().to(question_groups_list)))
}
