use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDWebhook},
    tsid::PalformDatabaseID,
};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm, entity_managers::webhooks::WebhookManager,
};

#[openapi(tag = "Webhooks", operation_id = "webhooks.delete")]
#[delete("/users/me/orgs/<_org_id>/forms/<form_id>/webhooks/<webhook_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    webhook_id: PalformDatabaseID<IDWebhook>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    WebhookManager::delete(db.inner(), form_id, webhook_id)
        .await
        .map_internal_error()?;

    Ok(())
}
