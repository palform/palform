use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::form::APIForm,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        rbac::{requests::APITokenOrgViewer, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::form_templates::FormTemplatesManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct CloneFormTemplateRequest {
    into_team: PalformDatabaseID<IDTeam>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormTemplatesClonePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    template_id: PalformDatabaseID<IDForm>,
}

#[api_operation(tag = "Form Templates", operation_id = "form_templates.clone")]
pub async fn form_templates_clone(
    path: Path<FormTemplatesClonePath>,
    data: Json<CloneFormTemplateRequest>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<Json<APIForm>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            &txn,
            data.into_team,
            path.org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    let new_form = FormTemplatesManager::clone(&txn, path.template_id, data.into_team)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::Form,
            Some(new_form.id.into_unknown()),
            Some(format!("Cloned from template {}", path.template_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(new_form))
}
