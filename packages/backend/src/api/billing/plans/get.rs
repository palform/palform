use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::plan::APIBillingSubscription, auth::rbac::requests::APITokenOrgAdmin,
    billing::manager::BillingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingPlansGetPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Plans", operation_id = "billing.plan.get")]
pub async fn billing_plans_get(
    path: Path<BillingPlansGetPath>,
    _token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIBillingSubscription>>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());
    let plans = manager
        .get_org_plans(db.as_ref(), path.org_id)
        .await
        .map_err(|e| APIError::report_internal_error("list org plans", e))?;

    Ok(Json(plans))
}
