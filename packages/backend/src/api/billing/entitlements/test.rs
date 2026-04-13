use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    auth::rbac::requests::APITokenOrgViewer,
    billing::entitlement::INTERNALBillingEntitlementManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingEntitlementsTestPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Entitlements", operation_id = "billing.entitlement.test")]
pub async fn billing_entitlements_test(
    path: Path<BillingEntitlementsTestPath>,
    data: Json<APIEntitlementRequest>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<bool>, APIError> {
    let manager = INTERNALBillingEntitlementManager::new(path.org_id);
    let resp = manager
        .check_entitlement(db.as_ref(), data.0)
        .await
        .map_err(|e| APIError::report_internal_error("check entitlement", e))?;
    Ok(Json(resp))
}
