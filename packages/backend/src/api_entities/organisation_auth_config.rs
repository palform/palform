use palform_entities::organisation_auth_config;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::{ActiveValue::NotSet, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, FromQueryResult, Clone)]
pub struct APIOrganisationAuthConfig {
    pub oidc_discovery_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub team_mapping_field_name: Option<String>,
    pub revoke_team_mappings: Option<bool>,
}

impl APIOrganisationAuthConfig {
    pub fn to_active_model(self) -> organisation_auth_config::ActiveModel {
        organisation_auth_config::ActiveModel {
            id: NotSet,
            oidc_discovery_url: Set(self.oidc_discovery_url),
            client_id: Set(self.client_id),
            client_secret: Set(self.client_secret),
            team_mapping_field_name: Set(self.team_mapping_field_name),
            revoke_team_mappings: Set(self.revoke_team_mappings),
        }
    }
}
