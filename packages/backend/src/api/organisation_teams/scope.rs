use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::organisation_teams::{
    create::organisation_teams_create, delete::organisation_teams_delete,
    get::organisation_teams_get, list::organisation_teams_list,
    list_my::organisation_teams_list_my,
};

pub fn organisation_teams_scope() -> Scope {
    scope("teams")
        .service(resource("my").route(get().to(organisation_teams_list_my)))
        .service(
            resource("")
                .route(get().to(organisation_teams_list))
                .route(post().to(organisation_teams_create)),
        )
        .service(
            scope("{team_id}").service(
                resource("")
                    .route(get().to(organisation_teams_get))
                    .route(delete().to(organisation_teams_delete)),
            ),
        )
}
