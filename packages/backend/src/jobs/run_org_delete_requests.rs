use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    actix_util::from_org_id::FromOrgIdTrait,
    config::Config,
    entity_managers::organisation_deletion_request::{
        OrgDeletionError, OrganisationDeletionRequestManager,
    },
};

pub async fn job_run_org_delete_requests(
    db: &DatabaseConnection,
    stripe: &stripe::Client,
    config: &Config,
) -> Result<(), OrgDeletionError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await?;

    let pending = OrganisationDeletionRequestManager::list_pending(&txn, config).await?;
    for request in pending {
        let manager = OrganisationDeletionRequestManager::new(request.organisation_id);
        manager.execute_request(&txn, &request, stripe).await?;
    }

    txn.commit().await?;
    Ok(())
}
