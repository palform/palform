use altcha::Challenge;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, ApiComponent)]
pub struct APICaptchaChallenge {
    #[serde(flatten)]
    #[schemars(with = "String")]
    pub challenge: Challenge,
}
