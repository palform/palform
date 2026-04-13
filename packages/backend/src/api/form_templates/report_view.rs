use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDForm, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::entity_managers::form_templates::FormTemplatesManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormTemplatesReportViewPath {
    template_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Form Templates", operation_id = "form_templates.report_view")]
pub async fn form_templates_report_view(
    path: Path<FormTemplatesReportViewPath>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    FormTemplatesManager::report_view(db.as_ref(), path.template_id)
        .await
        .map_internal_error()?;
    Ok(())
}
