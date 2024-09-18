use rocket::{get, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::Serialize;

use crate::{
    auth::social::{SocialAuthManager, SocialAuthService},
    config::Config,
};

#[derive(Serialize, JsonSchema)]
pub struct ListSocialAuthProvidersResponse {
    available_providers: Vec<SocialAuthService>,
}

#[openapi(tag = "Authentication", operation_id = "auth.social.list")]
#[get("/auth/social/providers")]
pub async fn handler(config: &State<Config>) -> Json<ListSocialAuthProvidersResponse> {
    Json(ListSocialAuthProvidersResponse {
        available_providers: SocialAuthManager::list_providers(config),
    })
}
