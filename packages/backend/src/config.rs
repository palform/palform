use std::str::FromStr;

use figment::{
    providers::{Env, Serialized},
    Figment,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::auth::social::SocialAuthService;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub cors_origin: String,
    pub frontend_url: Url,

    pub database_url: String,

    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_from_address: String,
    pub smtp_starttls: bool,
    pub smtp_skip_startup_check: bool,

    pub event_notification_address: String,

    pub s3_endpoint_url: String,
    pub s3_region: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,
    pub s3_team_assets_bucket: String,
    pub s3_submission_assets_bucket: String,
    pub s3_path_style: bool,

    pub captcha_secret_key: String,
    pub skip_captcha: bool,
    pub social_auth_providers: Vec<ConfigSocialAuthProvider>,
    pub auth_token_expiry_hours: u16,

    /// In bytes
    pub file_upload_size_limit: usize,

    #[cfg(feature = "saas")]
    pub stripe_secret_key: String,
    #[cfg(feature = "saas")]
    pub stripe_webhook_secret: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ConfigSocialAuthProvider {
    pub service: SocialAuthService,
    pub discovery_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    pub fn parse_config() -> Config {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Env::prefixed("PAL_").split("__").global())
            .extract()
            .unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bind_addr: "127.0.0.0:8000".to_string(),
            cors_origin: "^http:\\/\\/(\\w*\\.)?localhost:\\d{4}$".to_string(),
            frontend_url: Url::from_str("http://localhost:5173").unwrap(),
            database_url: "postgres://postgres:postgres@localhost:5432/palform".to_string(),
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: "user".to_string(),
            smtp_password: "password".to_string(),
            smtp_from_address: "noreply@palform.app".to_string(),
            smtp_starttls: true,
            smtp_skip_startup_check: false,
            event_notification_address: "admin@palform.app".to_string(),
            s3_endpoint_url: "https://s3.fr-par.scw.cloud".to_string(),
            s3_region: "fr-par".to_string(),
            s3_access_key: String::default(),
            s3_secret_key: String::default(),
            s3_team_assets_bucket: "team-assets".to_string(),
            s3_submission_assets_bucket: "submission-assets".to_string(),
            s3_path_style: true,
            captcha_secret_key: String::default(),
            skip_captcha: false,
            social_auth_providers: Vec::default(),
            auth_token_expiry_hours: 24,
            file_upload_size_limit: 52_428_800,
            stripe_secret_key: String::default(),
            stripe_webhook_secret: String::default(),
        }
    }
}
