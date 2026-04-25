use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::auth::second_factors::{
    delete::auth_second_factors_delete, enroll_totp::auth_second_factors_enroll_totp,
    enroll_webauthn::auth_second_factors_enroll_webauthn, list::auth_second_factors_list,
    start_webauthn::auth_second_factors_start_webauthn,
};

pub fn auth_second_factors_scope() -> Scope {
    scope("tfa")
        .service(resource("").route(get().to(auth_second_factors_list)))
        .service(resource("{factor_id}").route(delete().to(auth_second_factors_delete)))
        .service(
            scope("enroll")
                .service(resource("totp").route(post().to(auth_second_factors_enroll_totp)))
                .service(
                    resource("webauthn").route(post().to(auth_second_factors_enroll_webauthn)),
                ),
        )
        .service(
            scope("start")
                .service(resource("webauthn").route(post().to(auth_second_factors_start_webauthn))),
        )
}
