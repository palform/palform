use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, ApiComponent};
use chrono::{DateTime, Utc};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::AuditLogTargetResourceEnum;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::audit::APIAuditLogEntry, auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::audit::AuditEntityManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AuditLogListRequest {
    #[serde(default)]
    from: Option<DateTime<Utc>>,
    #[serde(default)]
    to: Option<DateTime<Utc>>,
    #[serde(default)]
    resource: Option<AuditLogTargetResourceEnum>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AuditLogListPath {
    pub org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Audit Logs", operation_id = "audit.list")]
pub async fn audit_logs_list(
    path: Path<AuditLogListPath>,
    data: Query<AuditLogListRequest>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIAuditLogEntry>>, APIError> {
    let resp = AuditEntityManager::list(
        db.as_ref(),
        path.org_id,
        data.from,
        data.to,
        data.resource.clone(),
    )
    .await
    .map_internal_error()?;

    Ok(Json(resp))
}
