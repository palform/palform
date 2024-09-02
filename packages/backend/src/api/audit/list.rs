use chrono::{DateTime, Utc};
use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AuditLogTargetResourceEnum;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::audit::APIAuditLogEntry, auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::audit::AuditEntityManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct AuditLogListRequest {
    #[serde(default)]
    from: Option<DateTime<Utc>>,
    #[serde(default)]
    to: Option<DateTime<Utc>>,
    #[serde(default)]
    resource: Option<AuditLogTargetResourceEnum>,
}

#[openapi(tag = "Audit Logs", operation_id = "audit.list")]
#[post("/user/me/orgs/<org_id>/audit", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<AuditLogListRequest>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIAuditLogEntry>>, APIErrorWithStatus> {
    let resp = AuditEntityManager::list(
        db.inner(),
        org_id,
        data.from,
        data.to,
        data.resource.clone(),
    )
    .await
    .map_internal_error()?;

    Ok(Json(resp))
}
