use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::webhook::APIWebhook, auth::rbac::requests::APITokenTeamViewerFromForm,
    entity_managers::webhooks::WebhookManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct WebhooksListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Webhooks", operation_id = "webhooks.list")]
pub async fn webhooks_list(
    path: Path<WebhooksListPath>,
    _token: APITokenTeamViewerFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIWebhook>>, APIError> {
    let webhooks = WebhookManager::list(db.as_ref(), path.form_id)
        .await
        .map_internal_error()?;
    Ok(Json(webhooks))
}
