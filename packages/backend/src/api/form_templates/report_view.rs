use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDForm, tsid::PalformDatabaseID};
use rocket::{post, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::entity_managers::form_templates::FormTemplatesManager;

#[openapi(tag = "Form Templates", operation_id = "form_templates.report_view")]
#[post("/templates/<template_id>/views")]
pub async fn handler(
    template_id: PalformDatabaseID<IDForm>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    FormTemplatesManager::report_view(db.inner(), template_id)
        .await
        .map_internal_error()?;
    Ok(())
}
