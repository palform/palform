use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_client_common::form_management::form_end::APIFormEndConfiguration;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDForm, IDFormBranding, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{
    AccessMode, ActiveModelTrait, DatabaseConnection, IntoActiveModel, IsolationLevel, Set,
    TransactionTrait,
};
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api::error::APIError;
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenTeamEditorFromForm;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::forms::FormManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateFormRequest {
    editor_name: String,
    title: Option<String>,
    branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    notification_email: bool,
    end_configuration: APIFormEndConfiguration,
    enable_captcha: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsUpdatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Forms", operation_id = "forms.update")]
pub async fn forms_update(
    path: Path<FormsUpdatePath>,
    data: Json<UpdateFormRequest>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if !FormManager::verify_form_org(&txn, path.form_id, path.org_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let mut form = FormManager::get_plain_by_id(&txn, path.form_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?
        .into_active_model();

    if data.enable_captcha {
        billing
            .check_entitlement(&txn, APIEntitlementRequest::FormCaptcha)
            .await?;
    }

    form.editor_name = Set(data.editor_name.clone());
    form.title = Set(data.title.clone());
    form.branding_id = Set(data.branding_id);
    form.notification_email = Set(data.notification_email);
    let end_config = FormManager::serialize_end_configuration(data.end_configuration.clone())
        .map_err(|e| APIError::BadRequest(e.to_string()))?;
    form.end_configuration = Set(end_config);
    form.enable_captcha = Set(data.enable_captcha);
    form.update(&txn).await.map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
