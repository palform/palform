use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
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
    auth::rbac::{requests::APITokenTeamAdminFromTeam, teams_manager::TeamsRBACManager},
    entity_managers::form_brandings::FormBrandingManager,
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
) -> Result<(), APIErrorWithStatus> {
    TeamsRBACManager::from(token.token)
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

    Ok(())
}
