use apistos::web::{get, resource, scope, Scope};

use crate::api::captcha::create::captcha_create;

pub fn captcha_scope() -> Scope {
    scope("captcha").service(resource("").route(get().to(captcha_create)))
}
