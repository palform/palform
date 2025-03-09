use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub struct APIFeedbackItem {
    pub score: i32,
    pub comment: Option<String>,
}
