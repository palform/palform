use apistos::web::{get, post, resource, scope, Scope};

use crate::api::team_assets::{
    get::team_assets_get, get_fill::team_assets_get_fill, list::team_assets_list,
    upload::team_assets_upload,
};

pub fn team_assets_scope() -> Scope {
    scope("assets")
        .service(
            resource("")
                .route(get().to(team_assets_list))
                .route(post().to(team_assets_upload)),
        )
        .service(resource("{asset_id}").route(get().to(team_assets_get)))
}

/// Fill-side asset fetch (`fill/orgs/{org_id}/forms/{form_id}/assets/{asset_id}`).
pub fn team_assets_fill_scope() -> Scope {
    scope("team_assets").service(resource("{asset_id}").route(get().to(team_assets_get_fill)))
}

/// Backwards-compatibility for images already embedded in question descriptions (pre Apr 2026)
pub fn team_assets_fill_legacy_scope() -> Scope {
    scope("assets").service(resource("{asset_id}").route(get().to(team_assets_get_fill)))
}
