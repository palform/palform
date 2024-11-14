use sea_orm::{DatabaseConnection, DbErr};

use crate::entity_managers::forms::FormManager;

pub async fn job_delete_old_submissions(db: &DatabaseConnection) -> Result<(), DbErr> {
    FormManager::delete_all_old_submissions(db).await
}
