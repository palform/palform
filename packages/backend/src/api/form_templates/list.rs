use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDFormTemplateCategory, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormTemplatesList {
    category_id: PalformDatabaseID<IDFormTemplateCategory>,
}

#[api_operation(tag = "Form Templates", operation_id = "form_templates.list")]
pub async fn form_templates_list(
    path: Path<FormTemplatesList>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplate>>, APIError> {
    let templates = FormTemplatesManager::list_in_category(db.as_ref(), path.category_id)
        .await
        .map_internal_error()?;
    Ok(Json(templates))
}
