use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::admin_users::APIServiceAccountAdminUser,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        admin_users::AdminUserManager, organisation_members::OrganisationMembersManager,
        organisation_teams::OrganisationTeamsManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AdminUsersServiceAccountsCreatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

/// Create a service account.
///
/// The service account will be added as a non-admin member of the organisation and a Viewer member
/// of its default team. This can be changed after creation.
#[api_operation(
    tag = "Admin Users",
    operation_id = "admin_users.service_accounts.create"
)]
pub async fn admin_users_service_accounts_create(
    path: Path<AdminUsersServiceAccountsCreatePath>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<Json<APIServiceAccountAdminUser>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let new_user = AdminUserManager::create_service_account_user(&txn)
        .await
        .map_err(|e| APIError::report_internal_error("Create service accont user", e))?;

    OrganisationMembersManager::create(&txn, path.org_id, new_user.id, false)
        .await
        .map_internal_error()?;

    let default_team = OrganisationTeamsManager::get_org_default_team(&txn, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    OrganisationTeamsManager::add_member_to_team(
        &txn,
        default_team.id,
        new_user.id,
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
            Some(new_user.id.into_unknown()),
            Some("New service account".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;

    Ok(Json(new_user))
}
