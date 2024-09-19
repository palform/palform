use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDFormTemplateCategory, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[openapi(tag = "Form Templates", operation_id = "form_templates.list")]
#[get("/templates/<category_id>/all", rank = 1)]
pub async fn handler(
    category_id: PalformDatabaseID<IDFormTemplateCategory>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplate>>, APIErrorWithStatus> {
    let templates = FormTemplatesManager::list_in_category(db.inner(), category_id)
        .await
        .map_internal_error()?;
    Ok(Json(templates))
}
