use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[openapi(tag = "Form Templates", operation_id = "form_templates.list_top")]
#[get("/templates/top")]
pub async fn handler(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplate>>, APIErrorWithStatus> {
    let top_templates = FormTemplatesManager::list_top_across_categories(db.inner(), 50)
        .await
        .map_internal_error()?;

    Ok(Json(top_templates))
}
