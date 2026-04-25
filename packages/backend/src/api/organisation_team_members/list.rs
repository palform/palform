use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::organisation_team::APIOrganisationTeamMember,
    auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamMembersListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Organisation Team Members", operation_id = "organisation.team.members.list")]
pub async fn organisation_team_members_list(
    path: Path<OrganisationTeamMembersListPath>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeamMember>>, APIError> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    if !OrganisationTeamsManager::verify_team_org(db.as_ref(), path.team_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    let resp = OrganisationTeamsManager::list_members_for_team(db.as_ref(), path.team_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(resp))
}
