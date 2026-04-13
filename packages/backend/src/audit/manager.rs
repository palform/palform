use std::marker::PhantomData;

use palform_entities::{
    audit_log_entry,
    sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum},
};
use palform_tsid::{
    resources::{IDAdminUser, IDAuditLogEntry, IDOrganisation, IDUnknown},
    tsid::PalformDatabaseID,
};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set};

use crate::{actix_util::from_org_id::FromOrgIdTrait, audit::id_chains::AuditLogIDChain};

pub struct AuditManager<Chain: AuditLogIDChain> {
    org_id: PalformDatabaseID<IDOrganisation>,
    chain: PhantomData<Chain>,
}

impl<Chain: AuditLogIDChain> FromOrgIdTrait for AuditManager<Chain> {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self {
            org_id,
            chain: PhantomData,
        }
    }
}

impl<Chain: AuditLogIDChain> AuditManager<Chain> {
    pub async fn log_event_with_id_chain_and_note<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
        note: Option<String>,
        id_chain: Option<Chain>,
    ) -> Result<(), DbErr> {
        let new_entry = audit_log_entry::ActiveModel {
            id: Set(PalformDatabaseID::<IDAuditLogEntry>::random()),
            user_id: Set(user_id),
            verb: Set(verb),
            target_resource_type: Set(resource_type),
            target_resource_id: Set(resource_id),
            note: Set(note),
            organisation_id: Set(self.org_id),
            target_resource_parent_ids: Set(id_chain.map(|v| v.to_vec()).unwrap_or_default()),
            ..Default::default()
        };

        new_entry.insert(conn).await.map(|_| ())
    }

    pub async fn log_event_with_id_chain<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
        id_chain: Option<Chain>,
    ) -> Result<(), DbErr> {
        self.log_event_with_id_chain_and_note(
            conn,
            user_id,
            verb,
            resource_type,
            resource_id,
            None,
            id_chain,
        )
        .await
    }

    pub async fn log_event_with_note<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
        note: Option<String>,
    ) -> Result<(), DbErr> {
        self.log_event_with_id_chain_and_note(
            conn,
            user_id,
            verb,
            resource_type,
            resource_id,
            note,
            None,
        )
        .await
    }

    pub async fn log_event<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        verb: AuditLogVerbEnum,
        resource_type: AuditLogTargetResourceEnum,
        resource_id: Option<PalformDatabaseID<IDUnknown>>,
    ) -> Result<(), DbErr> {
        self.log_event_with_id_chain_and_note(
            conn,
            user_id,
            verb,
            resource_type,
            resource_id,
            None,
            None,
        )
        .await
    }
}
