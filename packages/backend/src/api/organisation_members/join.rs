use chrono::Utc;
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::resources::{IDOrganisation, IDOrganisationInvite};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{post, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::api::error::{APIError, APIInternalError};
use crate::api_entities::billing::entitlement::APIEntitlementRequest;
use crate::audit::AuditManager;
use crate::auth::tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourcePersonal};
use crate::entity_managers::billing_entitlement_proxy::BillingEntitlementManager;
use crate::entity_managers::organisation_auth_config::OrganisationAuthConfigManager;
use crate::entity_managers::organisation_invites::OrganisationInviteManager;
use crate::entity_managers::organisation_members::OrganisationMembersManager;
use crate::entity_managers::organisation_teams::OrganisationTeamsManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(JsonSchema, Deserialize)]
pub struct JoinOrganisationRequest {
    pub invite_id: PalformDatabaseID<IDOrganisationInvite>,
}

#[openapi(
    tag = "Organisation Members",
    operation_id = "organisation.members.join"
)]
#[post("/users/me/orgs/<org_id>/members", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<JoinOrganisationRequest>,
    token: APIAuthToken<APIAuthTokenSourcePersonal>,
    audit: FromOrgId<AuditManager>,
    db: &State<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
    auth: FromOrgId<OrganisationAuthConfigManager>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    if auth.check_org_uses_oidc(&txn).await.map_internal_error()? {
        return Err(APIError::BadRequest(
            "Cannot join organisation that uses OIDC for login".to_string(),
        )
        .into());
    }

    let invite = OrganisationInviteManager::get_by_id(&txn, data.invite_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if invite.organisation_id != org_id {
        return Err(APIError::NotFound.into());
    }

    if invite.expires_at < Utc::now().naive_utc() {
        return Err(APIError::BadRequest("Invite expired".to_string()).into());
    }

    if OrganisationMembersManager::check_is_member(&txn, org_id, token.get_user_id())
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::BadRequest(
            "You are already a member of that organisation".to_string(),
        )
        .into());
    }

    billing
        .check_entitlement(&txn, APIEntitlementRequest::UserCount)
        .await?;

    if invite.single_use {
        OrganisationInviteManager::delete(&txn, invite.id)
            .await
            .map_err(|e| e.to_internal_error())?;
    }

    OrganisationMembersManager::create(&txn, org_id, token.get_user_id(), false)
        .await
        .map_err(|e| e.to_internal_error())?;

    let default_team = OrganisationTeamsManager::get_org_default_team(&txn, org_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    OrganisationTeamsManager::add_member_to_team(
        &txn,
        default_team.id,
        token.get_user_id(),
        OrganisationMemberRoleEnum::Viewer,
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(token.get_user_id().into_unknown()),
            Some(format!("Joined via invite {}", invite.id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(())
}
