use apistos::web::{get, post, resource, scope, Scope};

use crate::api::form_templates::{
    clone::form_templates_clone, get::form_templates_get,
    get_category::form_templates_get_category, list::form_templates_list,
    list_categories::form_templates_list_categories, list_top::form_templates_list_top,
    report_view::form_templates_report_view,
};

pub fn form_templates_public_scope() -> Scope {
    scope("templates")
        .service(resource("top").route(get().to(form_templates_list_top)))
        .service(
            scope("categories")
                .service(resource("").route(get().to(form_templates_list_categories)))
                .service(
                    scope("{category_id}")
                        .service(resource("").route(get().to(form_templates_get_category)))
                        .service(resource("all").route(get().to(form_templates_list))),
                ),
        )
        .service(
            scope("{template_id}")
                .service(resource("").route(get().to(form_templates_get)))
                .service(resource("views").route(post().to(form_templates_report_view))),
        )
}

pub fn form_templates_clone_scope() -> Scope {
    scope("templates")
        .service(resource("{template_id}/clone").route(post().to(form_templates_clone)))
}
