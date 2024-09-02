use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_brandings::APIFormBrandingAccess,
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::form_brandings::FormBrandingManager,
};

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.list_access"
)]
#[get("/users/me/orgs/<_org_id>/teams/<team_id>/brandings/<branding_id>/access")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
    _token: APITokenTeamViewerFromTeam,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFormBrandingAccess>>, APIErrorWithStatus> {
    if !FormBrandingManager::verify_branding_team_allowed(db.inner(), branding_id, team_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let accessing_teams = FormBrandingManager::list_accessing_teams(db.inner(), branding_id)
        .await
        .map_internal_error()?;
    Ok(Json(accessing_teams))
}
