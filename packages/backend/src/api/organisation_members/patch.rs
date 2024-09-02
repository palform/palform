use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::resources::{IDAdminUser, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::serde::json::Json;
use rocket::{patch, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::{DatabaseConnection, DbErr};
use serde::Deserialize;

use crate::audit::AuditManager;
use crate::auth::rbac::requests::APITokenOrgAdmin;
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::organisation_members::OrganisationMembersManager;
use crate::rocket_util::from_org_id::FromOrgId;

#[derive(Deserialize, JsonSchema)]
pub struct PatchOrgMemberRequest {
    is_admin: bool,
}

#[openapi(
    tag = "Organisation Members",
    operation_id = "organisation.members.patch"
)]
#[patch("/users/me/orgs/<org_id>/members/<user_id>", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    user_id: PalformDatabaseID<IDAdminUser>,
    data: Json<PatchOrgMemberRequest>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), APIErrorWithStatus> {
    OrganisationMembersManager::set_is_admin(db.inner(), org_id, user_id, data.is_admin)
        .await
        .map_err(|e| match e {
            DbErr::RecordNotFound(_) => APIError::NotFound.into(),
            _ => APIError::report_internal_error("set member is_admin", e),
        })?;

    audit
        .log_event_with_note(
            db.inner(),
            token.token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationMember,
            Some(user_id.into_unknown()),
            Some(format!("Set is_admin to {}", data.is_admin)),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
