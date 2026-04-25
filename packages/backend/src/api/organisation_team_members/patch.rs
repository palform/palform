use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainTeamMember, manager::AuditManager},
    auth::{
        rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
        tokens::APIAuthTokenSource,
    },
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamMembersPatchPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    member_user_id: PalformDatabaseID<IDAdminUser>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub(crate) struct OrganisationTeamMembersPatchRequest {
    new_role: OrganisationMemberRoleEnum,
}

#[api_operation(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.patch"
)]
pub async fn organisation_team_members_patch(
    path: Path<OrganisationTeamMembersPatchPath>,
    data: Json<OrganisationTeamMembersPatchRequest>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    audit: FromOrgId<AuditManager<IDChainTeamMember>>,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let token_user_id = if let Some(org_admin_token) = org_admin_token {
        org_admin_token.get_user_id()
    } else if let Some(team_admin_token) = team_admin_token {
        team_admin_token.get_user_id()
    } else {
        return Err(APIError::NotAllowed.into());
    };

    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !OrganisationTeamsManager::verify_team_org(&txn, path.team_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !OrganisationTeamsManager::verify_is_member(&txn, path.team_id, path.member_user_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    OrganisationTeamsManager::change_member_role(
        &txn,
        path.team_id,
        path.member_user_id,
        data.new_role.clone(),
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_id_chain_and_note(
            &txn,
            token_user_id,
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::TeamMember,
            Some(path.member_user_id.into_unknown()),
            Some(format!(
                "Set role to {}",
                serde_json::to_string(&data.new_role).map_err(|e| {
                    APIError::report_internal_error("Serialize new team member role", e)
                })?
            )),
            Some(IDChainTeamMember::new(path.team_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
