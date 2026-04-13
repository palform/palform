use actix_web::web::{Data, Json, Path, Query};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api_entities::billing::invoice::APIBillingUpcomingInvoice;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::billing::manager::BillingManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SwitchPlanRequest {
    new_stripe_price_id: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SwitchPlanQuery {
    dry_run: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct BillingPlansSwitchPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Billing Plans", operation_id = "billing.plan.switch")]
pub async fn billing_plans_switch(
    path: Path<BillingPlansSwitchPath>,
    data: Json<SwitchPlanRequest>,
    Query(query): Query<SwitchPlanQuery>,
    token: APITokenOrgAdmin,
    stripe: Data<stripe::Client>,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<Json<Option<APIBillingUpcomingInvoice>>, APIError> {
    let manager = BillingManager::new(stripe.as_ref());

    if query.dry_run {
        let preview_invoice = manager
            .preview_plan_change(db.as_ref(), path.org_id, data.new_stripe_price_id.clone())
            .await
            .map_err(|e| {
                APIError::report_internal_error("preview change to subscription plan", e)
            })?;

        Ok(Json(Some(preview_invoice)))
    } else {
        manager
            .change_subscription_plan(db.as_ref(), path.org_id, data.new_stripe_price_id.clone())
            .await
            .map_err(|e| APIError::report_internal_error("change subscription plan", e))?;

        audit
            .log_event_with_note(
                db.as_ref(),
                token.get_user_id(),
                AuditLogVerbEnum::Update,
                AuditLogTargetResourceEnum::Organisation,
                Some(path.org_id.into_unknown()),
                Some("Change subscription plan".to_string()),
            )
            .await
            .map_internal_error()?;
        Ok(Json(None))
    }
}
