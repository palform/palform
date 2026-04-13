use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::organisation_invites::{
    create::organisation_invites_create, delete::organisation_invites_delete,
    list::organisation_invites_list, preview::organisation_invites_preview,
};

pub fn organisation_invites_scope() -> Scope {
    scope("invites")
        .service(
            resource("")
                .route(get().to(organisation_invites_list))
                .route(post().to(organisation_invites_create)),
        )
        .service(
            scope("{invite_id}")
                .service(resource("preview").route(get().to(organisation_invites_preview)))
                .service(resource("").route(delete().to(organisation_invites_delete))),
        )
}
