use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::fill_tokens::{
    create::fill_tokens_create, delete::fill_tokens_delete, list::fill_tokens_list,
};

pub fn fill_tokens_scope() -> Scope {
    scope("fill_access_tokens")
        .service(
            resource("")
                .route(get().to(fill_tokens_list))
                .route(post().to(fill_tokens_create)),
        )
        .service(resource("{token_id}").route(delete().to(fill_tokens_delete)))
}
