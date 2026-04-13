use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::auth::{
    auth_callback::auth_callback, create_user::auth_create_user, invalidate::auth_invalidate,
    password_reset::scope::auth_password_reset_scope,
    resend_verification::auth_resend_verification,
    second_factors::scope::auth_second_factors_scope, sign_in::auth_sign_in,
    social::scope::auth_social_scope, start_auth::auth_start_auth, test::auth_test,
    verify_email::auth_verify_email, verify_tfa::auth_verify_tfa,
};

pub fn auth_scope() -> Scope {
    scope("auth")
        .service(auth_social_scope())
        .service(auth_password_reset_scope())
        .service(auth_second_factors_scope())
        .service(resource("signup").route(post().to(auth_create_user)))
        .service(
            scope("signin")
                .service(resource("signin").route(post().to(auth_sign_in)))
                .service(resource("verify_tfa").route(post().to(auth_verify_tfa))),
        )
        .service(scope("tokens").service(resource("current").route(delete().to(auth_invalidate))))
        .service(resource("test").route(get().to(auth_test)))
        .service(
            scope("verification")
                .service(resource("resend").route(post().to(auth_resend_verification)))
                .service(resource("{verification_id}").route(post().to(auth_verify_email))),
        )
        .service(
            scope("{org_id}")
                .service(resource("callback").route(post().to(auth_callback)))
                .service(resource("start").route(post().to(auth_start_auth))),
        )
}
