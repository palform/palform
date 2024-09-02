use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::{resources::{IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::form_brandings::APIFormBranding,
    auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
    entity_managers::form_brandings::FormBrandingManager,
};

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.list"
)]
#[get("/users/me/orgs/<_org_id>/teams/<team_id>/brandings")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIFormBranding>>, APIErrorWithStatus> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    FormBrandingManager::list_in_team(db.inner(), team_id)
        .await
        .map(|v| Json(v))
        .map_internal_error()
}
