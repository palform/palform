use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::{IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    audit::AuditManager, auth::rbac::requests::APITokenTeamAdminFromTeam,
    auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_teams::OrganisationTeamsManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(tag = "Organisation Teams", operation_id = "organisation.teams.delete")]
#[delete("/users/me/orgs/<_org_id>/teams/<team_id>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    token: APITokenTeamAdminFromTeam,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let team = OrganisationTeamsManager::get_by_id(&txn, team_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    if team.is_default.is_some_and(|x| x) {
        return Err(APIError::BadRequest("Cannot delete default team".to_string()).into());
    }

    OrganisationTeamsManager::delete(&txn, team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Team,
            Some(team_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
