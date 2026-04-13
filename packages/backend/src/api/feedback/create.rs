use actix_web::web::{Data, Json};
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::feedback_items::APIFeedbackItem, entity_managers::feedback::FeedbackManager,
};

#[api_operation(tag = "Feedback", operation_id = "feedback.create")]
pub async fn feedback_create(
    data: Json<APIFeedbackItem>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    FeedbackManager::create_feedback_item(db.as_ref(), data.score, data.comment.to_owned())
        .await
        .map_internal_error()?;

    Ok(())
}
