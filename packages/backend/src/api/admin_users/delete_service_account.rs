use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
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
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        admin_users::AdminUserManager, organisation_members::OrganisationMembersManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct AdminUsersServiceAccountsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    admin_user_id: PalformDatabaseID<IDAdminUser>,
}

/// Delete a service account.
#[api_operation(
    tag = "Admin Users",
    operation_id = "admin_users.service_accounts.delete"
)]
pub async fn admin_users_service_accounts_delete(
    path: Path<AdminUsersServiceAccountsDeletePath>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let user = AdminUserManager::get_service_account_user_by_id(&txn, path.admin_user_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;

    let is_member = OrganisationMembersManager::check_is_member(&txn, path.org_id, user.id)
        .await
        .map_internal_error()?;
    if !is_member {
        return Err(APIError::NotFound);
    }

    AdminUserManager::delete_user(&txn, user.id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(path.admin_user_id.into_unknown()),
            Some("Deleted service account".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;

    Ok(())
}
