use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIErrorWithStatus, APIInternalError},
    api_entities::organisation_team::APIOrganisationTeamMembership,
    auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[openapi(
    tag = "Organisation Teams",
    operation_id = "organisation.teams.list_my"
)]
#[get("/users/me/orgs/<org_id>/teams.my")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeamMembership>>, APIErrorWithStatus> {
    let teams =
        OrganisationTeamsManager::list_member_teams(db.inner(), org_id, token.get_user_id())
            .await
            .map_err(|e| e.to_internal_error())?;

    Ok(Json(teams))
}
