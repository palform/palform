use apistos::web::{delete, get, patch, post, put, resource, scope, Scope};

use crate::api::forms::{
    create::forms_create, delete::forms_delete, exchange_short_link::forms_exchange_short_link,
    fill::forms_fill, get::forms_get, keys::forms_keys, list::forms_list, relocate::forms_relocate,
    set_auto_delete::forms_set_auto_delete, update::forms_update, view::forms_view,
};

pub fn forms_org_scope() -> Scope {
    scope("forms")
        .service(
            resource("")
                .route(get().to(forms_list))
                .route(post().to(forms_create)),
        )
        .service(
            scope("{form_id}")
                .service(
                    resource("")
                        .route(get().to(forms_get))
                        .route(put().to(forms_update))
                        .route(delete().to(forms_delete)),
                )
                .service(resource("location").route(patch().to(forms_relocate)))
                .service(resource("auto-delete").route(put().to(forms_set_auto_delete))),
        )
}

/// Public fill routes (`/fill/forms/...`).
pub fn forms_fill_scope() -> Scope {
    scope("forms")
        .service(
            scope("short_link").service(
                scope("{subdomain}")
                    .service(resource("{short_link}").route(get().to(forms_exchange_short_link))),
            ),
        )
        .service(
            scope("{form_id}/org/{org_id}")
                .service(
                    resource("")
                        .route(get().to(forms_view))
                        .route(post().to(forms_fill)),
                )
                .service(resource("keys").route(get().to(forms_keys))),
        )
}
