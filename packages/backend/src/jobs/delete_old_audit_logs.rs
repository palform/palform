use sea_orm::{DatabaseConnection, DbErr};

use crate::entity_managers::audit::AuditEntityManager;

pub async fn job_delete_old_audit_logs(db: &DatabaseConnection) -> Result<(), DbErr> {
    AuditEntityManager::delete_old_entries(db).await
}
