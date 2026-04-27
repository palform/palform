use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::organisation_deletion_requests::{
    cancel::organisation_deletion_requests_cancel, list::organisation_deletion_requests_list,
    skip::organisation_deletion_requests_skip,
};

pub fn organisation_deletion_requests_scope() -> Scope {
    scope("deletion_requests")
        .service(resource("").route(get().to(organisation_deletion_requests_list)))
        .service(
            scope("{request_id}")
                .service(resource("").route(delete().to(organisation_deletion_requests_cancel)))
                .service(resource("skip").route(post().to(organisation_deletion_requests_skip))),
        )
}
