use apistos::{info::Info, spec::Spec};

use crate::auth::tokens::{http_security, SECURITY_PERSONAL, SECURITY_SERVICE_ACCOUNT};

pub fn get_openapi_spec() -> Spec {
    Spec {
        info: Info {
            title: "Palform".to_owned(),
            ..Default::default()
        },
        // Routes without a token must set `no_security` to opt out of this
        // If that is not set, it means either of the two sources is allowed,
        // which is the case for most routes.
        securities: vec![
            http_security(
                SECURITY_PERSONAL,
                "basic",
                "Only for Palform's internal requests from its own frontend",
            ),
            http_security(
                SECURITY_SERVICE_ACCOUNT,
                "bearer",
                "Service account tokens created for your organisation",
            ),
        ],
        ..Default::default()
    }
}
