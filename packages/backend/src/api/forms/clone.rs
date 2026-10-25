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
        rbac::{requests::APITokenTeamViewerFromForm, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsClonePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormsCloneRequest {
    /// The team to place the new form into.
    new_team_id: PalformDatabaseID<IDTeam>,
}

/// Creates a copy of a form and its question groups and questions.
///
/// The copy is created in the specified team. The user calling this method must have read
/// permissions on the form (i.e. the `Viewer` role in its team) and the `Editor` role on the team
/// into which the form is being copied.
///
/// The responses and the fill access tokens of the form will not be copied.
#[api_operation(tag = "Forms", operation_id = "forms.clone")]
pub async fn forms_clone(
    path: Path<FormsClonePath>,
    data: Json<FormsCloneRequest>,
    token: APITokenTeamViewerFromForm,
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

    // Membership of current team is already verified, we just need to check the target team
    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            &txn,
            data.new_team_id,
            path.org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    let new_form = FormManager::clone(&txn, path.form_id, data.new_team_id, true)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::Form,
            Some(new_form.id.into_unknown()),
            Some(format!("Cloned from {}", path.form_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;

    Ok(Json(new_form))
}
