use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct AlertResponse {
    alert_type: AlertType,
    hide_on: Vec<HideContext>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct InductionAlertPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Induction", operation_id = "induction.alert")]
pub async fn induction_alert(
    path: Path<InductionAlertPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Option<AlertResponse>>, APIError> {
    let manager = InductionStatusManager::new(token.get_user_id(), path.org_id, db.as_ref());
    let has_active_key = manager.has_active_key().await.map_internal_error()?;
    if !has_active_key {
        return Ok(Json(Some(AlertResponse {
            alert_type: AlertType::NoActiveKey,
            hide_on: vec![HideContext::Induction, HideContext::Keys],
        })));
    }

    Ok(Json(None))
}
