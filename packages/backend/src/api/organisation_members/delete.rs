use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{delete, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::APIError,
    audit::AuditManager,
    auth::rbac::requests::APITokenOrgAdmin,
    auth::tokens::APIAuthTokenSource,
    entity_managers::{
        keys::UserKeyManager, organisation_members::OrganisationMembersManager,
        organisation_teams::OrganisationTeamsManager,
    },
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Members",
    operation_id = "organisation.members.delete"
)]
#[delete("/users/me/orgs/<org_id>/members/<user_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    user_id: PalformDatabaseID<IDAdminUser>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    OrganisationMembersManager::delete(&txn, org_id, user_id)
        .await
        .map_internal_error()?;

    OrganisationTeamsManager::remove_from_all_teams(&txn, org_id, user_id)
        .await
        .map_internal_error()?;

    UserKeyManager::delete_all_in_org_for_user(&txn, org_id, user_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(user_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
