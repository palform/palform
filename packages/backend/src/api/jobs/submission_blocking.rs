use chrono::{Duration, NaiveTime};
use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use rocket::{post, State};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    billing::{entitlement::INTERNALBillingEntitlementManager, manager::BillingManager},
    entity_managers::{orgs::OrganisationManager, submission::SubmissionManager},
};

#[post("/jobs/submission-blocking")]
pub async fn handler(
    db: &State<DatabaseConnection>,
    stripe: &State<stripe::Client>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let all_orgs = OrganisationManager::list_all_orgs(&txn)
        .await
        .map_internal_error()?;

    let billing = BillingManager::new(stripe);
    for org_id in all_orgs {
        let (period_start, is_annual) = billing
            .get_org_billing_month_start(&txn, org_id)
            .await
            .map_err(|e| APIError::report_internal_error("get org billing month", e))?;

        let submission_count = SubmissionManager::total_submission_count_in_org(
            &txn,
            org_id,
            period_start.naive_utc(),
        )
        .await
        .map_internal_error()?;

        let billing_entitlement = INTERNALBillingEntitlementManager::new(org_id);
        let entitlement = billing_entitlement
            .get_entitlement_info(&txn)
            .await
            .map_err(|e| APIError::report_internal_error("get entitlement for org", e))?;

        let limit_reached = entitlement.response_count.is_some_and(|l| {
            if is_annual {
                submission_count >= ((l * 12) as u64)
            } else {
                submission_count >= (l as u64)
            }
        });

        OrganisationManager::set_org_submission_block(
            &txn,
            org_id,
            if limit_reached {
                Some(
                    (period_start + Duration::days(30))
                        .with_time(NaiveTime::from_hms_opt(23, 59, 59).unwrap())
                        .unwrap()
                        .naive_utc(),
                )
            } else {
                None
            },
        )
        .await
        .map_internal_error()?;
    }

    txn.commit().await.map_internal_error()?;
    Ok(())
}
