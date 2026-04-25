use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::resources::{IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::organisation_teams::OrganisationTeamsManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct CreateTeamRequest {
    name: String,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamsCreatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Teams", operation_id = "organisation.teams.create")]
pub async fn organisation_teams_create(
    path: Path<OrganisationTeamsCreatePath>,
    data: Json<CreateTeamRequest>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<PalformDatabaseID<IDTeam>>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    billing
        .check_entitlement(&txn, APIEntitlementRequest::TeamCount)
        .await?;

    let new_team = OrganisationTeamsManager::create(&txn, path.org_id, data.name.clone(), false)
        .await
        .map_internal_error()?;

    OrganisationTeamsManager::add_member_to_team(
        &txn,
        new_team.id,
        token.get_user_id(),
        OrganisationMemberRoleEnum::Admin,
    )
    .await
    .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::Team,
            Some(new_team.id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(new_team.id))
}
