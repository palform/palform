use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDWebhook},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm, entity_managers::webhooks::WebhookManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct CreateWebhookRequest {
    pub endpoint: Url,
}

#[derive(Serialize, JsonSchema)]
pub struct CreateWebhookResponse {
    pub id: PalformDatabaseID<IDWebhook>,
    pub signing_secret: String,
}

#[openapi(tag = "Webhooks", operation_id = "webhooks.create")]
#[post("/users/me/orgs/<_org_id>/forms/<form_id>/webhooks", data = "<data>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Json<CreateWebhookRequest>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<Json<CreateWebhookResponse>, APIErrorWithStatus> {
    WebhookManager::validate_url(data.endpoint.clone())
        .map_err(|e| APIError::ValidationError(format!("Invalid URL: {}", e)))?;

    WebhookManager::test_connection(data.endpoint.clone())
        .await
        .map_err(|e| APIError::ValidationError(format!("TCP connection test: {}", e)))?;

    let new_webhook = WebhookManager::create(db.inner(), form_id, data.endpoint.clone())
        .await
        .map_internal_error()?;

    Ok(Json(CreateWebhookResponse {
        id: new_webhook.id,
        signing_secret: new_webhook.signing_secret,
    }))
}
