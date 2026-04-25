use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::organisation_auth_team_mappings::{
    create::organisation_auth_team_mappings_create, delete::organisation_auth_team_mappings_delete,
    list::organisation_auth_team_mappings_list,
};

pub fn organisation_auth_team_mappings_scope() -> Scope {
    scope("mappings")
        .service(
            resource("")
                .route(get().to(organisation_auth_team_mappings_list))
                .route(post().to(organisation_auth_team_mappings_create)),
        )
        .service(
            resource("{mapping_id}").route(delete().to(organisation_auth_team_mappings_delete)),
        )
}
