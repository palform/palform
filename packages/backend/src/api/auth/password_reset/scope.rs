use apistos::web::{post, put, resource, scope, Scope};

use crate::api::auth::password_reset::{
    reset::auth_password_reset_reset, send::auth_password_reset_send,
};

pub fn auth_password_reset_scope() -> Scope {
    scope("password_reset")
        .service(resource("send").route(post().to(auth_password_reset_send)))
        .service(resource("reset").route(put().to(auth_password_reset_reset)))
}
