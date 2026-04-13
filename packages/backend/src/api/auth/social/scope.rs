use apistos::web::{get, post, resource, scope, Scope};

use crate::api::auth::social::{
    callback::auth_social_callback, list::auth_social_list_providers, start::auth_social_start,
};

pub fn auth_social_scope() -> Scope {
    scope("social")
        .service(resource("providers").route(get().to(auth_social_list_providers)))
        .service(resource("start").route(post().to(auth_social_start)))
        .service(resource("callback").route(post().to(auth_social_callback)))
}
