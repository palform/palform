use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
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
pub struct OrganisationTeamMembersDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    member_user_id: PalformDatabaseID<IDAdminUser>,
}

#[api_operation(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.delete"
)]
pub async fn organisation_team_members_delete(
    path: Path<OrganisationTeamMembersDeletePath>,
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

    if !OrganisationTeamsManager::verify_is_member(&txn, path.team_id, path.member_user_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    if !OrganisationTeamsManager::verify_team_org(&txn, path.team_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    OrganisationTeamsManager::remove_from_team(&txn, path.team_id, path.member_user_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_id_chain(
            &txn,
            token_user_id,
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::TeamMember,
            Some(path.member_user_id.into_unknown()),
            Some(IDChainTeamMember::new(path.team_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
