use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::api_entities::billing::invoice::APIBillingUpcomingInvoice;
use crate::audit::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::billing::manager::BillingManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(Deserialize, JsonSchema)]
pub struct SwitchPlanRequest {
    new_stripe_price_id: String,
}

#[openapi(tag = "Billing Plans", operation_id = "billing.plan.switch")]
#[post(
    "/users/me/orgs/<org_id>/billing/plan.switch?<dry_run>",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<SwitchPlanRequest>,
    dry_run: bool,
    token: APITokenOrgAdmin,
    stripe: &State<stripe::Client>,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<Json<Option<APIBillingUpcomingInvoice>>, APIErrorWithStatus> {
    let manager = BillingManager::new(stripe);

    if dry_run {
        let preview_invoice = manager
            .preview_plan_change(db.inner(), org_id, data.new_stripe_price_id.clone())
            .await
            .map_err(|e| {
                APIError::report_internal_error("preview change to subscription plan", e)
            })?;

        Ok(Json(Some(preview_invoice)))
    } else {
        manager
            .change_subscription_plan(db.inner(), org_id, data.new_stripe_price_id.clone())
            .await
            .map_err(|e| APIError::report_internal_error("change subscription plan", e))?;

        audit
            .log_event_with_note(
                db.inner(),
                token.get_user_id(),
                AuditLogVerbEnum::Update,
                AuditLogTargetResourceEnum::Organisation,
                Some(org_id.into_unknown()),
                Some("Change subscription plan".to_string()),
            )
            .await
            .map_internal_error()?;
        Ok(Json(None))
    }
}
