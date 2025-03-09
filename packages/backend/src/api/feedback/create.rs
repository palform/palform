use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::feedback_items::APIFeedbackItem, entity_managers::feedback::FeedbackManager,
};

#[openapi(tag = "Feedback", operation_id = "feedback.create")]
#[post("/feedback", data = "<data>")]
pub async fn handler(
    data: Json<APIFeedbackItem>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    FeedbackManager::create_feedback_item(db.inner(), data.score, data.comment.to_owned())
        .await
        .map_internal_error()?;

    Ok(())
}
