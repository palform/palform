use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalError};
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::resources::{IDAdminUser, IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam};
use crate::entity_managers::organisation_members::OrganisationMembersManager;
use crate::entity_managers::organisation_teams::OrganisationTeamsManager;

#[derive(Deserialize, JsonSchema)]
pub struct AddTeamMemberRequest {
    user_ids: Vec<PalformDatabaseID<IDAdminUser>>,
    role: OrganisationMemberRoleEnum,
}

#[openapi(
    tag = "Organisation Team Members",
    operation_id = "organisation.team.members.add"
)]
#[post("/users/me/orgs/<org_id>/teams/<team_id>/members", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    data: Json<AddTeamMemberRequest>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if !OrganisationTeamsManager::verify_team_org(&txn, team_id, org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    for user_id in data.user_ids.clone() {
        if !OrganisationMembersManager::check_is_member(&txn, org_id, user_id)
            .await
            .map_err(|e| e.to_internal_error())?
        {
            return Err(
                APIError::BadRequest(format!("User {} not in organisation", user_id)).into(),
            );
        }

        OrganisationTeamsManager::add_member_to_team(&txn, team_id, user_id, data.role.clone())
            .await
            .map_err(|e| e.to_internal_error())?;
    }

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
