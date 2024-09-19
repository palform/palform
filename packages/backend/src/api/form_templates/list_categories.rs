use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplateCategory,
    entity_managers::form_templates::FormTemplatesManager,
};

#[openapi(
    tag = "Form Templates",
    operation_id = "form_templates.categories.list"
)]
#[get("/templates")]
pub async fn handler(
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplateCategory>>, APIErrorWithStatus> {
    let categories = FormTemplatesManager::list_categories(db.inner())
        .await
        .map_internal_error()?;

    Ok(Json(categories))
}
