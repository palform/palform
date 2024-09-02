use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::billing::entitlement::APIEntitlementInfo,
    auth::rbac::requests::APITokenOrgViewer,
    billing::entitlement::INTERNALBillingEntitlementManager,
};

#[openapi(
    tag = "Billing Entitlements",
    operation_id = "billing.entitlement.list"
)]
#[get("/users/me/orgs/<org_id>/billing/entitlements")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIEntitlementInfo>, APIErrorWithStatus> {
    let manager = INTERNALBillingEntitlementManager::new(org_id);
    let resp = manager
        .get_entitlement_info(db.inner())
        .await
        .map_internal_error()?;
    Ok(Json(resp))
}
