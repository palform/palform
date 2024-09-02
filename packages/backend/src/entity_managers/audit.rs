use chrono::{DateTime, Duration, Utc};
use palform_entities::{
    admin_user, audit_log_entry, prelude::*, sea_orm_active_enums::AuditLogTargetResourceEnum,
};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType, Order, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait,
};

use crate::api_entities::audit::APIAuditLogEntry;

pub struct AuditEntityManager;

impl AuditEntityManager {
    pub async fn list<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        target_type: Option<AuditLogTargetResourceEnum>,
    ) -> Result<Vec<APIAuditLogEntry>, sea_orm::prelude::DbErr> {
        let mut condition =
            Condition::all().add(audit_log_entry::Column::OrganisationId.eq(org_id));
        if let Some(from) = from {
            condition = condition.add(audit_log_entry::Column::CreatedAt.gt(from));
        }
        if let Some(to) = to {
            condition = condition.add(audit_log_entry::Column::CreatedAt.lt(to));
        }
        if let Some(target_type) = target_type {
            condition = condition.add(audit_log_entry::Column::TargetResourceType.eq(target_type));
        }

        AuditLogEntry::find()
            .filter(condition)
            .order_by(audit_log_entry::Column::CreatedAt, Order::Desc)
            .join(
                JoinType::InnerJoin,
                audit_log_entry::Relation::AdminUser.def(),
            )
            .column_as(admin_user::Column::DisplayName, "user_display_name")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn delete_old_entries<T: ConnectionTrait>(conn: &T) -> Result<(), DbErr> {
        AuditLogEntry::delete_many()
            .filter(audit_log_entry::Column::CreatedAt.lt(Utc::now() - Duration::days(28)))
            .exec(conn)
            .await
            .map(|_| ())
    }
}
