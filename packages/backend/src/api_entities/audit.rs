use chrono::NaiveDateTime;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminUser, IDAuditLogEntry, IDUnknown};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, FromQueryResult, JsonSchema)]
pub struct APIAuditLogEntry {
    id: PalformDatabaseID<IDAuditLogEntry>,
    user_id: PalformDatabaseID<IDAdminUser>,
    user_display_name: String,
    verb: AuditLogVerbEnum,
    target_resource_type: AuditLogTargetResourceEnum,
    target_resource_id: PalformDatabaseID<IDUnknown>,
    created_at: NaiveDateTime,
    note: Option<String>,
}
