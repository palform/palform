use crate::config::Config;

pub fn init_stripe_client(config: &Config) -> stripe::Client {
    stripe::Client::new(config.stripe_secret_key.clone())
}
