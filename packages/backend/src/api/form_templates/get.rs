use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDForm, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormTemplatesGetPath {
    template_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Form Templates", operation_id = "form_templates.get")]
pub async fn form_templates_get(
    path: Path<FormTemplatesGetPath>,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIFormTemplate>, APIError> {
    let form = FormTemplatesManager::get(db.as_ref(), path.template_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;
    Ok(Json(form))
}
