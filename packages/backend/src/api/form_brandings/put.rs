use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::{IDFormBranding, IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{put, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_brandings::APIFormBrandingRequest,
    auth::rbac::requests::APITokenTeamEditorFromTeam,
    entity_managers::form_brandings::FormBrandingManager,
};

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.put"
)]
#[put(
    "/users/me/orgs/<_org_id>/teams/<team_id>/brandings/<branding_id>",
    data = "<data>"
)]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
    data: Json<APIFormBrandingRequest>,
    _token: APITokenTeamEditorFromTeam,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    FormBrandingManager::verify_branding_team_allowed(db.inner(), branding_id, team_id)
        .await
        .map_internal_error()?;

    FormBrandingManager::update(db.inner(), branding_id, data.0)
        .await
        .map_internal_error()?;

    Ok(())
}
