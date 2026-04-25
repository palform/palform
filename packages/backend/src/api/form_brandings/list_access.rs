use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::form_brandings::APIFormBrandingAccess,
    auth::rbac::requests::APITokenTeamViewerFromTeam,
    entity_managers::form_brandings::FormBrandingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormBrandingsListAccessPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
}

#[api_operation(tag = "Form Brandings", operation_id = "organisation.team.branding.list_access")]
pub async fn form_brandings_list_access(
    path: Path<FormBrandingsListAccessPath>,
    _token: APITokenTeamViewerFromTeam,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIFormBrandingAccess>>, APIError> {
    if !FormBrandingManager::verify_branding_team_allowed(
        db.as_ref(),
        path.branding_id,
        path.team_id,
    )
    .await
    .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    let accessing_teams = FormBrandingManager::list_accessing_teams(db.as_ref(), path.branding_id)
        .await
        .map_internal_error()?;
    Ok(Json(accessing_teams))
}
