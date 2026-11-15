use apistos::web::{delete, get, post, resource, scope, Scope};

use crate::api::legal_agreements::{
    create::legal_agreements_create, delete::legal_agreements_delete, list::legal_agreements_list,
    list_agreement_types::legal_agreements_list_agreement_types,
    variables::legal_agreements_variables_list,
};

pub fn legal_agreements_scope() -> Scope {
    scope("legal_agreements")
        .service(resource("").route(get().to(legal_agreements_list)))
        .service(resource("types").route(get().to(legal_agreements_list_agreement_types)))
        .service(
            resource("variables/{agreement_type}")
                .route(get().to(legal_agreements_variables_list))
                .route(post().to(legal_agreements_create)),
        )
        .service(resource("{agreement_id}").route(delete().to(legal_agreements_delete)))
}
