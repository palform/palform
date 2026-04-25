use actix_web::web::{Data, Json, Path};
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
    api_entities::webhook::APIWebhookJob,
    auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::{webhook_jobs::WebhookJobsManager, webhooks::WebhookManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct WebhooksListJobsPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    webhook_id: PalformDatabaseID<IDWebhook>,
}

#[api_operation(tag = "Webhooks", operation_id = "webhooks.list_jobs")]
pub async fn webhooks_list_jobs(
    path: Path<WebhooksListJobsPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIWebhookJob>>, APIError> {
    if !WebhookManager::verify_form(db.as_ref(), path.webhook_id, path.form_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let wjm = WebhookJobsManager::new(db.as_ref());
    let jobs = wjm.list(path.webhook_id).await.map_internal_error()?;
    Ok(Json(jobs))
}
