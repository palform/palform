use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamAdminFromTeam, tokens::APIAuthTokenSource},
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamsDeletePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Organisation Teams", operation_id = "organisation.teams.delete")]
pub async fn organisation_teams_delete(
    path: Path<OrganisationTeamsDeletePath>,
    token: APITokenTeamAdminFromTeam,
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

    let team = OrganisationTeamsManager::get_by_id(&txn, path.team_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    if team.is_default.is_some_and(|x| x) {
        return Err(APIError::BadRequest("Cannot delete default team".to_string()).into());
    }

    OrganisationTeamsManager::delete(&txn, path.team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Team,
            Some(path.team_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
