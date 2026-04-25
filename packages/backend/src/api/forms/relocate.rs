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
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        rbac::{requests::APITokenTeamEditorFromForm, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsRelocatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsRelocateRequest {
    target_team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Forms", operation_id = "forms.relocate")]
pub async fn forms_relocate(
    path: Path<FormsRelocatePath>,
    data: Json<FormsRelocateRequest>,
    token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    // Membership of current team is already verified, we just need to check the target team
    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            &txn,
            data.target_team_id,
            path.org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    FormManager::change_form_team(&txn, path.form_id, data.target_team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Form,
            Some(path.form_id.into_unknown()),
            Some(format!("Moved from to team {}", data.target_team_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
