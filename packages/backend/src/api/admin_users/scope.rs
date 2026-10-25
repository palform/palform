use apistos::web::{delete, patch, post, resource, scope, Scope};

use crate::api::admin_users::{
    create_service_account::admin_users_service_accounts_create,
    delete_service_account::admin_users_service_accounts_delete, update::admin_users_update,
};

pub fn admin_users_global_scope() -> Scope {
    scope("/users").service(resource("/me").route(patch().to(admin_users_update)))
}

pub fn admin_users_org_scope() -> Scope {
    scope("/users").service(
        scope("/service-accounts")
            .service(resource("").route(post().to(admin_users_service_accounts_create)))
            .service(
                resource("{admin_user_id}").route(delete().to(admin_users_service_accounts_delete)),
            ),
    )
}
