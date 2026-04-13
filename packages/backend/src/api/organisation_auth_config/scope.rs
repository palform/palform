use apistos::web::{get, put, resource, scope, Scope};

use crate::api::organisation_auth_config::{get::organisation_auth_config_get, put::organisation_auth_config_put};

pub fn organisation_auth_config_scope() -> Scope {
    scope("auth_config")
        .service(
            resource("")
                .route(get().to(organisation_auth_config_get))
                .route(put().to(organisation_auth_config_put)),
        )
}
