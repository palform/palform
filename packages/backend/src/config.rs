use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub cors_origin: String,

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

    pub captcha_secret_key: String,
    pub skip_captcha: bool,

    #[cfg(feature = "saas")]
    pub stripe_secret_key: String,
    #[cfg(feature = "saas")]
    pub stripe_webhook_secret: String,
}

impl Config {
    pub fn parse_config() -> Config {
        Figment::new()
            .merge(Env::prefixed("PAL_").split("__").global())
            .extract()
            .unwrap()
    }
}
