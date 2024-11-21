use crate::entity_managers::webhook_jobs::WebhookJobsManager;
use sea_orm::{AccessMode, DatabaseConnection, DbErr, IsolationLevel, TransactionTrait};

pub async fn job_run_webhooks(db: &DatabaseConnection) -> Result<(), DbErr> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let mut wjm = WebhookJobsManager::new(&txn);
    wjm.run_all_jobs().await?;

    txn.commit().await?;
    Ok(())
}
