use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::resources::{IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::fairing::Result;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::audit::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::organisation_teams::OrganisationTeamsManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(Deserialize, JsonSchema)]
pub struct CreateTeamRequest {
    name: String,
}

#[openapi(tag = "Organisation Teams", operation_id = "organisation.teams.create")]
#[post("/users/me/orgs/<org_id>/teams", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<CreateTeamRequest>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<PalformDatabaseID<IDTeam>>, APIErrorWithStatus> {
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

    let new_team = OrganisationTeamsManager::create(&txn, org_id, data.name.clone(), false)
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
