use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminUser, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::{DatabaseConnection, DbErr};
use serde::Deserialize;

use crate::actix_util::from_org_id::FromOrgId;
use crate::audit::id_chains::IDChainEmpty;
use crate::audit::manager::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::organisation_members::OrganisationMembersManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct PatchOrgMemberRequest {
    is_admin: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationMembersPatchPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    user_id: PalformDatabaseID<IDAdminUser>,
}

#[api_operation(
    tag = "Organisation Members",
    operation_id = "organisation.members.patch"
)]
pub async fn organisation_members_patch(
    path: Path<OrganisationMembersPatchPath>,
    data: Json<PatchOrgMemberRequest>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    OrganisationMembersManager::set_is_admin(db.as_ref(), path.org_id, path.user_id, data.is_admin)
        .await
        .map_err(|e| match e {
            DbErr::RecordNotFound(_) => APIError::NotFound.into(),
            _ => APIError::report_internal_error("set member is_admin", e),
        })?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(path.user_id.into_unknown()),
            Some(format!("Set is_admin to {}", data.is_admin)),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
