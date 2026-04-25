use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::APIInternalError, api_entities::organisation_team::APIOrganisationTeamMembership,
    auth::rbac::requests::APITokenOrgViewer, auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationTeamsListMyPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Teams", operation_id = "organisation.teams.list_my")]
pub async fn organisation_teams_list_my(
    path: Path<OrganisationTeamsListMyPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationTeamMembership>>, APIError> {
    let teams =
        OrganisationTeamsManager::list_member_teams(db.as_ref(), path.org_id, token.get_user_id())
            .await
            .map_err(|e| e.to_internal_error())?;

    Ok(Json(teams))
}
