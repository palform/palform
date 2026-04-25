use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::entitlement::APIEntitlementInfo,
    auth::rbac::requests::APITokenOrgViewer,
    billing::entitlement::INTERNALBillingEntitlementManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingEntitlementsListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Entitlements", operation_id = "billing.entitlement.list")]
pub async fn billing_entitlements_list(
    path: Path<BillingEntitlementsListPath>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIEntitlementInfo>, APIError> {
    let manager = INTERNALBillingEntitlementManager::new(path.org_id);
    let resp = manager
        .get_entitlement_info(db.as_ref())
        .await
        .map_internal_error()?;
    Ok(Json(resp))
}
