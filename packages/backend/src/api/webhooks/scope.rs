use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::webhooks::{
    create::webhooks_create, delete::webhooks_delete, list::webhooks_list,
    list_jobs::webhooks_list_jobs,
};

pub fn webhooks_scope() -> Scope {
    scope("webhooks")
        .service(
            resource("")
                .route(get().to(webhooks_list))
                .route(post().to(webhooks_create)),
        )
        .service(
            scope("{webhook_id}")
                .service(resource("jobs").route(get().to(webhooks_list_jobs)))
                .service(resource("").route(delete().to(webhooks_delete))),
        )
}
