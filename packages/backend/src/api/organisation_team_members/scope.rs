use apistos::web::{delete, get, patch, post, resource, scope, Scope};

use crate::api::organisation_team_members::{
    add::organisation_team_members_add, delete::organisation_team_members_delete,
    list::organisation_team_members_list, patch::organisation_team_members_patch,
};

pub fn organisation_team_members_scope() -> Scope {
    scope("members")
        .service(
            resource("")
                .route(get().to(organisation_team_members_list))
                .route(post().to(organisation_team_members_add)),
        )
        .service(
            resource("{member_user_id}")
                .route(patch().to(organisation_team_members_patch))
                .route(delete().to(organisation_team_members_delete)),
        )
}
