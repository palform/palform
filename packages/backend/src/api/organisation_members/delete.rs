use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::APIError,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        keys::UserKeyManager, organisation_members::OrganisationMembersManager,
        organisation_teams::OrganisationTeamsManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationMembersDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    user_id: PalformDatabaseID<IDAdminUser>,
}

#[api_operation(
    tag = "Organisation Members",
    operation_id = "organisation.members.delete"
)]
pub async fn organisation_members_delete(
    path: Path<OrganisationMembersDeletePath>,
    token: APITokenOrgAdmin,
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

    OrganisationMembersManager::delete(&txn, path.org_id, path.user_id)
        .await
        .map_internal_error()?;

    OrganisationTeamsManager::remove_from_all_teams(&txn, path.org_id, path.user_id)
        .await
        .map_internal_error()?;

    UserKeyManager::delete_all_in_org_for_user(&txn, path.org_id, path.user_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(path.user_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
