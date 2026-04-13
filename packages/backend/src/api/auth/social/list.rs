use actix_web::web::{Data, Json};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::Serialize;

use crate::{
    auth::social::{SocialAuthManager, SocialAuthService},
    config::Config,
};

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct ListSocialAuthProvidersResponse {
    available_providers: Vec<SocialAuthService>,
}

#[api_operation(tag = "Authentication", operation_id = "auth.social.list")]
pub async fn auth_social_list_providers(
    config: Data<Config>,
) -> Json<ListSocialAuthProvidersResponse> {
    Json(ListSocialAuthProvidersResponse {
        available_providers: SocialAuthManager::list_providers(config.as_ref()),
    })
}
