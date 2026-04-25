use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDWebhook},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm, entity_managers::webhooks::WebhookManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct WebhooksDeletePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    webhook_id: PalformDatabaseID<IDWebhook>,
}

#[api_operation(tag = "Webhooks", operation_id = "webhooks.delete")]
pub async fn webhooks_delete(
    path: Path<WebhooksDeletePath>,
    _token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    WebhookManager::delete(db.as_ref(), path.form_id, path.webhook_id)
        .await
        .map_internal_error()?;

    Ok(())
}
