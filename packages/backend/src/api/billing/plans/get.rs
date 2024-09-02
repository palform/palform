use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::plan::APIBillingSubscription, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[openapi(tag = "Billing Plans", operation_id = "billing.plan.get")]
#[get("/users/me/orgs/<org_id>/billing/plan")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIBillingSubscription>>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);
    let plans = manager
        .get_org_plans(db.inner(), org_id)
        .await
        .map_err(|e| APIError::report_internal_error("list org plans", e))?;

    Ok(Json(plans))
}
