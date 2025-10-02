use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_brandings::APIFormBrandingAccess,
    audit::AuditManager,
    auth::{
        rbac::{requests::APITokenTeamAdminFromTeam, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::form_brandings::FormBrandingManager,
    rocket_util::from_org_id::FromOrgId,
};

#[derive(Deserialize, JsonSchema)]
pub struct AddAccessRequest {
    for_team_id: PalformDatabaseID<IDTeam>,
}

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.add_access"
)]
#[post(
    "/users/me/orgs/<org_id>/teams/<team_id>/brandings/<branding_id>/access",
    data = "<data>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
    data: Json<AddAccessRequest>,
    token: APITokenTeamAdminFromTeam,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<Json<APIFormBrandingAccess>, APIErrorWithStatus> {
    if !FormBrandingManager::verify_branding_team_allowed(db.inner(), branding_id, team_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            db.inner(),
            data.for_team_id,
            org_id,
            OrganisationMemberRoleEnum::Admin,
        )
        .await?;

    let access = FormBrandingManager::add_access(db.inner(), branding_id, data.for_team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Branding,
            Some(branding_id.into_unknown()),
            Some(format!("Added access for team {}", data.for_team_id)),
        )
        .await
        .map_internal_error()?;

    Ok(Json(access))
}
