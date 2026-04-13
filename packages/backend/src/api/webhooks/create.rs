use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDWebhook},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm, entity_managers::webhooks::WebhookManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct CreateWebhookRequest {
    pub endpoint: Url,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct CreateWebhookResponse {
    pub id: PalformDatabaseID<IDWebhook>,
    pub signing_secret: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct WebhooksCreatePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Webhooks", operation_id = "webhooks.create")]
pub async fn webhooks_create(
    path: Path<WebhooksCreatePath>,
    data: Json<CreateWebhookRequest>,
    _token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
) -> Result<Json<CreateWebhookResponse>, APIError> {
    WebhookManager::validate_url(data.endpoint.clone())
        .map_err(|e| APIError::ValidationError(format!("Invalid URL: {}", e)))?;

    WebhookManager::test_connection(data.endpoint.clone())
        .await
        .map_err(|e| APIError::ValidationError(format!("TCP connection test: {}", e)))?;

    let new_webhook = WebhookManager::create(db.as_ref(), path.form_id, data.endpoint.clone())
        .await
        .map_internal_error()?;

    Ok(Json(CreateWebhookResponse {
        id: new_webhook.id,
        signing_secret: new_webhook.signing_secret,
    }))
}
