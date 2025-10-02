use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{delete, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    audit::AuditManager,
    auth::{
        rbac::{requests::APITokenTeamAdminFromTeam, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::form_brandings::FormBrandingManager,
    rocket_util::from_org_id::FromOrgId,
};

#[derive(Deserialize, JsonSchema)]
pub struct DeleteAccessRequest {
    for_team_id: PalformDatabaseID<IDTeam>,
}

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.delete_access"
)]
#[delete(
    "/users/me/orgs/<org_id>/teams/<_team_id>/brandings/<branding_id>/access",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
    data: Json<DeleteAccessRequest>,
    token: APITokenTeamAdminFromTeam,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<(), APIErrorWithStatus> {
    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            db.inner(),
            data.for_team_id,
            org_id,
            OrganisationMemberRoleEnum::Admin,
        )
        .await?;

    FormBrandingManager::remove_access(db.inner(), branding_id, data.for_team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Branding,
            Some(branding_id.into_unknown()),
            Some(format!("Removed access for team {}", data.for_team_id)),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
