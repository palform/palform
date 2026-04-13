use actix_web::web::{Data, Json};
use apistos::api_operation;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[api_operation(tag = "Form Templates", operation_id = "form_templates.list_top")]
pub async fn form_templates_list_top(
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFormTemplate>>, APIError> {
    let top_templates = FormTemplatesManager::list_top_across_categories(db.as_ref(), 50)
        .await
        .map_internal_error()?;

    Ok(Json(top_templates))
}
