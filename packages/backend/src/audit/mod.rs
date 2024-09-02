use palform_entities::{
    audit_log_entry,
    sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum},
};
use palform_tsid::{
    resources::{IDAdminUser, IDAuditLogEntry, IDOrganisation, IDUnknown},
    tsid::PalformDatabaseID,
};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};

use crate::rocket_util::from_org_id::FromOrgIdTrait;

pub struct AuditManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for AuditManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }
}

impl AuditManager {
    pub async fn log_event_with_note<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
        note: Option<String>,
    ) -> Result<(), DbErr> {
        let new_entry = audit_log_entry::ActiveModel {
            id: Set(PalformDatabaseID::<IDAuditLogEntry>::random()),
            user_id: Set(user_id),
            verb: Set(verb),
            target_resource_type: Set(resource_type),
            target_resource_id: Set(resource_id),
            note: Set(note),
            organisation_id: Set(self.org_id),
            ..Default::default()
        };

        new_entry.insert(conn).await.map(|_| ())
    }

    pub async fn log_event<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
    ) -> Result<(), DbErr> {
        self.log_event_with_note(conn, user_id, verb, resource_type, resource_id, None)
            .await
    }
}
