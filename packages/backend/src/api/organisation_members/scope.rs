use apistos::web::{delete, get, patch, post, resource, scope, Scope};

use crate::api::organisation_members::{
    am_i_admin::organisation_members_am_i_admin, delete::organisation_members_delete,
    join::organisation_members_join, list::organisation_members_list,
    patch::organisation_members_patch,
};

pub fn organisation_members_scope() -> Scope {
    scope("members")
        .service(resource("am-i-admin").route(get().to(organisation_members_am_i_admin)))
        .service(
            resource("")
                .route(get().to(organisation_members_list))
                .route(post().to(organisation_members_join)),
        )
        .service(
            resource("{user_id}")
                .route(patch().to(organisation_members_patch))
                .route(delete().to(organisation_members_delete)),
        )
}
