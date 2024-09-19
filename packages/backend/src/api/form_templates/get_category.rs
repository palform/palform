use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDFormTemplateCategory, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplateCategory,
    entity_managers::form_templates::FormTemplatesManager,
};

#[openapi(tag = "Form Templates", operation_id = "form_templates.categories.get")]
#[get("/templates/categories/<category_id>")]
pub async fn handler(
    category_id: PalformDatabaseID<IDFormTemplateCategory>,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIFormTemplateCategory>, APIErrorWithStatus> {
    let category = FormTemplatesManager::get_category(db.inner(), category_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    Ok(Json(category))
}
