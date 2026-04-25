use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_brandings::APIFormBranding,
    auth::rbac::requests::{APITokenOrgAdmin, APITokenTeamAdminFromTeam},
    entity_managers::form_brandings::FormBrandingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormBrandingsListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(tag = "Form Brandings", operation_id = "organisation.team.branding.list")]
pub async fn form_brandings_list(
    path: Path<FormBrandingsListPath>,
    org_admin_token: Option<APITokenOrgAdmin>,
    team_admin_token: Option<APITokenTeamAdminFromTeam>,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFormBranding>>, APIError> {
    if org_admin_token.is_none() && team_admin_token.is_none() {
        return Err(APIError::NotAllowed.into());
    }

    FormBrandingManager::list_in_team(db.as_ref(), path.team_id)
        .await
        .map(Json)
        .map_internal_error()
}
