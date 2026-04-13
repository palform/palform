use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDFormTemplateCategory, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_template::APIFormTemplateCategory,
    entity_managers::form_templates::FormTemplatesManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormTemplatesGetCategoryPath {
    category_id: PalformDatabaseID<IDFormTemplateCategory>,
}

#[api_operation(tag = "Form Templates", operation_id = "form_templates.categories.get")]
pub async fn form_templates_get_category(
    path: Path<FormTemplatesGetCategoryPath>,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIFormTemplateCategory>, APIError> {
    let category = FormTemplatesManager::get_category(db.as_ref(), path.category_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    Ok(Json(category))
}
