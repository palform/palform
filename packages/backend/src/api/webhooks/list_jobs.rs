use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDWebhook},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::webhook::APIWebhookJob,
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{webhook_jobs::WebhookJobsManager, webhooks::WebhookManager},
};

#[openapi(tag = "Webhooks", operation_id = "webhooks.list_jobs")]
#[get("/users/me/orgs/<_org_id>/forms/<form_id>/webhooks/<webhook_id>/jobs")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    webhook_id: PalformDatabaseID<IDWebhook>,
    _token: APITokenTeamViewerFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIWebhookJob>>, APIErrorWithStatus> {
    if !WebhookManager::verify_form(db.inner(), webhook_id, form_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let wjm = WebhookJobsManager::new(db.inner());
    let jobs = wjm.list(webhook_id).await.map_internal_error()?;
    Ok(Json(jobs))
}
