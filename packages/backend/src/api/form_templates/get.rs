use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDForm, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_template::APIFormTemplate,
    entity_managers::form_templates::FormTemplatesManager,
};

#[openapi(tag = "Form Templates", operation_id = "form_templates.get")]
#[get("/templates/<template_id>")]
pub async fn handler(
    template_id: PalformDatabaseID<IDForm>,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIFormTemplate>, APIErrorWithStatus> {
    let form = FormTemplatesManager::get(db.inner(), template_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;
    Ok(Json(form))
}
