use altcha::Challenge;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, ApiComponent)]
#[serde(transparent)]
pub struct APICaptchaChallenge {
    #[schemars(with = "serde_json::Map<String, serde_json::Value>")]
    pub challenge: Challenge,
}
