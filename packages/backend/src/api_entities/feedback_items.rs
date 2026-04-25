use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct APIFeedbackItem {
    pub score: i32,
    pub comment: Option<String>,
}
