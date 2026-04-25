use apistos::{info::Info, spec::Spec};

pub fn get_openapi_spec() -> Spec {
    Spec {
        info: Info {
            title: "Palform".to_owned(),
            ..Default::default()
        },
        ..Default::default()
    }
}
