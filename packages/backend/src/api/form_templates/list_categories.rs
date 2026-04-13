use actix_web::web::{Data, Json};
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplateCategory,
    entity_managers::form_templates::FormTemplatesManager,
};

#[api_operation(tag = "Form Templates", operation_id = "form_templates.categories.list")]
pub async fn form_templates_list_categories(
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplateCategory>>, APIError> {
    let categories = FormTemplatesManager::list_categories(db.as_ref())
        .await
        .map_internal_error()?;

    Ok(Json(categories))
}
