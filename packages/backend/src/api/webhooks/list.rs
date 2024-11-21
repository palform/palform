use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::webhook::APIWebhook, auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::webhooks::WebhookManager,
};

#[openapi(tag = "Webhooks", operation_id = "webhooks.list")]
#[get("/users/me/orgs/<_org_id>/forms/<form_id>/webhooks")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIWebhook>>, APIErrorWithStatus> {
    let webhooks = WebhookManager::list(db.inner(), form_id)
        .await
        .map_internal_error()?;
    Ok(Json(webhooks))
}
