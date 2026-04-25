use apistos::web::{delete, get, post, put, resource, scope, Scope};

use crate::api::keys::{
    delete::keys_delete, get::keys_get, get_backup::keys_get_backup, list::keys_list,
    list_org::keys_list_org, register::keys_register, register_backup::keys_register_backup,
    team_fingerprints::keys_team_fingerprints,
};

pub fn organisation_keys_scope() -> Scope {
    scope("keys")
        .service(
            resource("my")
                .route(get().to(keys_list))
                .route(post().to(keys_register)),
        )
        .service(resource("all").route(get().to(keys_list_org)))
        .service(
            scope("{key_id}")
                .service(
                    resource("")
                        .route(get().to(keys_get))
                        .route(delete().to(keys_delete)),
                )
                .service(
                    resource("backup")
                        .route(get().to(keys_get_backup))
                        .route(put().to(keys_register_backup)),
                ),
        )
}

/// Team key fingerprints (`teams/{team_id}/keys/all`).
pub fn organisation_team_keys_scope() -> Scope {
    scope("keys").service(resource("all").route(get().to(keys_team_fingerprints)))
}
