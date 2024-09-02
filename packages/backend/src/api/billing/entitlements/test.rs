use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    auth::rbac::requests::APITokenOrgViewer,
    billing::entitlement::INTERNALBillingEntitlementManager,
};

#[openapi(
    tag = "Billing Entitlements",
    operation_id = "billing.entitlement.test"
)]
#[post("/users/me/orgs/<org_id>/billing/entitlements/test", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<APIEntitlementRequest>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<bool>, APIErrorWithStatus> {
    let manager = INTERNALBillingEntitlementManager::new(org_id);
    let resp = manager
        .check_entitlement(db.inner(), data.0)
        .await
        .map_err(|e| APIError::report_internal_error("check entitlement", e))?;
    Ok(Json(resp))
}
