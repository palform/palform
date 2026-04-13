use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminUser, IDAuditLogEntry, IDUnknown};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, FromQueryResult, JsonSchema, ApiComponent)]
pub struct APIAuditLogEntry {
    id: PalformDatabaseID<IDAuditLogEntry>,
    user_id: PalformDatabaseID<IDAdminUser>,
    user_display_name: Option<String>,
    verb: AuditLogVerbEnum,
    target_resource_type: AuditLogTargetResourceEnum,
    target_resource_id: PalformDatabaseID<IDUnknown>,
    target_resource_parent_ids: Vec<String>,
    created_at: DateTime<Utc>,
    note: Option<String>,
}
