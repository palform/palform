use apistos::web::{delete, get, post, put, resource, scope, Scope};

use crate::api::form_brandings::{
    add_access::form_brandings_add_access, create::form_brandings_create,
    delete::form_brandings_delete, list::form_brandings_list,
    list_access::form_brandings_list_access, list_fonts::form_brandings_list_fonts,
    put::form_brandings_put, remove_access::form_brandings_remove_access,
};

pub fn form_brandings_scope() -> Scope {
    scope("brandings")
        .service(
            resource("")
                .route(get().to(form_brandings_list))
                .route(post().to(form_brandings_create)),
        )
        .service(
            scope("{branding_id}")
                .service(
                    resource("")
                        .route(put().to(form_brandings_put))
                        .route(delete().to(form_brandings_delete)),
                )
                .service(
                    resource("access")
                        .route(get().to(form_brandings_list_access))
                        .route(post().to(form_brandings_add_access))
                        .route(delete().to(form_brandings_remove_access)),
                ),
        )
}

pub fn form_brandings_google_fonts_scope() -> Scope {
    scope("fonts").service(resource("").route(get().to(form_brandings_list_fonts)))
}
