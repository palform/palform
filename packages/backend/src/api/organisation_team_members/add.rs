use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::resources::{IDAdminUser, IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::audit::id_chains::IDChainTeamMember;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam};
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::organisation_members::OrganisationMembersManager;
use crate::entity_managers::organisation_teams::OrganisationTeamsManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AddTeamMemberRequest {
    user_ids: Vec<PalformDatabaseID<IDAdminUser>>,
    role: OrganisationMemberRoleEnum,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamMembersAddPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.add"
)]
pub async fn organisation_team_members_add(
    path: Path<OrganisationTeamMembersAddPath>,
    data: Json<AddTeamMemberRequest>,
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

    for user_id in data.user_ids.clone() {
        if !OrganisationMembersManager::check_is_member(&txn, path.org_id, user_id)
            .await
            .map_err(|e| e.to_internal_error())?
        {
            return Err(
                APIError::BadRequest(format!("User {} not in organisation", user_id)).into(),
            );
        }

        OrganisationTeamsManager::add_member_to_team(
            &txn,
            path.team_id,
            user_id,
            data.role.clone(),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

        audit
            .log_event_with_id_chain(
                &txn,
                token_user_id,
                AuditLogVerbEnum::Create,
                AuditLogTargetResourceEnum::TeamMember,
                Some(user_id.into_unknown()),
                Some(IDChainTeamMember::new(path.team_id)),
            )
            .await
            .map_internal_error()?;
    }

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
