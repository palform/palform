use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{get, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::auth::rbac::requests::APITokenOrgViewer;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::induction::InductionStatusManager;

#[derive(Serialize, JsonSchema)]
pub enum AlertType {
    NoActiveKey,
}

#[derive(Serialize, JsonSchema)]
pub enum HideContext {
    Induction,
    Keys,
}

#[derive(Serialize, JsonSchema)]
pub struct AlertResponse {
    alert_type: AlertType,
    hide_on: Vec<HideContext>,
}

#[openapi(tag = "Induction", operation_id = "induction.alert")]
#[get("/users/me/orgs/<org_id>/induction/alert")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Option<AlertResponse>>, APIErrorWithStatus> {
    let manager = InductionStatusManager::new(token.get_user_id(), org_id, db.inner());
    let has_active_key = manager.has_active_key().await.map_internal_error()?;
    if !has_active_key {
        return Ok(Json(Some(AlertResponse {
            alert_type: AlertType::NoActiveKey,
            hide_on: vec![HideContext::Induction, HideContext::Keys],
        })));
    }

    Ok(Json(None))
}
