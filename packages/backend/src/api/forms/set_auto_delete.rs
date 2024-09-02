use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::{IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{put, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::AuditManager,
    auth::rbac::requests::APITokenTeamEditorFromForm,
    auth::tokens::APIAuthTokenSource,
    entity_managers::{billing_entitlement_proxy::BillingEntitlementManager, forms::FormManager},
    rocket_util::from_org_id::FromOrgId,
};

#[derive(Deserialize, JsonSchema)]
pub struct SetSubmissionAutoDeleteRequest {
    days: Option<i32>,
}

#[openapi(tag = "Forms", operation_id = "forms.set_auto_delete")]
#[put(
    "/users/me/orgs/<_org_id>/forms/<form_id>/auto-delete",
    data = "<data>"
)]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Json<SetSubmissionAutoDeleteRequest>,
    token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIErrorWithStatus> {
    billing
        .check_entitlement(db.inner(), APIEntitlementRequest::SubmissionAutoDelete)
        .await?;

    FormManager::set_auto_delete(db.inner(), form_id, data.days)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(form_id.into_unknown()),
            Some(format!(
                "Set submission auto-delete to {}",
                data.days
                    .map(|v| format!("{} day(s)", v))
                    .unwrap_or("off".to_string())
            )),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
