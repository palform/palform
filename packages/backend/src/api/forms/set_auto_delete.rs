use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::billing::entitlement::APIEntitlementRequest,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamEditorFromForm, tokens::APIAuthTokenSource},
    entity_managers::{billing_entitlement_proxy::BillingEntitlementManager, forms::FormManager},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SetSubmissionAutoDeleteRequest {
    days: Option<i32>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsSetAutoDeletePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Forms", operation_id = "forms.set_auto_delete")]
pub async fn forms_set_auto_delete(
    path: Path<FormsSetAutoDeletePath>,
    data: Json<SetSubmissionAutoDeleteRequest>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIError> {
    billing
        .check_entitlement(db.as_ref(), APIEntitlementRequest::SubmissionAutoDelete)
        .await?;

    FormManager::set_auto_delete(db.as_ref(), path.form_id, data.days)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
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
