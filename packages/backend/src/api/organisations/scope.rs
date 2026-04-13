use apistos::web::{delete, get, patch, post, resource, scope, Scope};

use crate::api::organisations::{
    create::organisations_create, create_subdomain::organisations_create_subdomain,
    delete::organisations_delete, get::organisations_get, list::organisations_list,
    patch::organisations_patch, resolve_subdomain::organisations_resolve_subdomain,
};

pub fn organisations_scope() -> Scope {
    scope("orgs")
        .service(resource("for-subdomain").route(get().to(organisations_resolve_subdomain)))
        .service(
            resource("")
                .route(get().to(organisations_list))
                .route(post().to(organisations_create)),
        )
        .service(
            scope("{org_id}")
                .service(
                    resource("")
                        .route(get().to(organisations_get))
                        .route(patch().to(organisations_patch))
                        .route(delete().to(organisations_delete)),
                )
                .service(resource("subdomain").route(post().to(organisations_create_subdomain))),
        )
}
