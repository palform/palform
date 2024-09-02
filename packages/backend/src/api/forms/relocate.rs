use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{resources::{IDForm, IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use rocket::{patch, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    auth::rbac::{requests::APITokenTeamEditorFromForm, teams_manager::TeamsRBACManager},
    entity_managers::forms::FormManager,
};

#[openapi(tag = "Forms", operation_id = "forms.relocate")]
#[patch(
    "/users/me/orgs/<org_id>/forms/<form_id>/location",
    data = "<target_team_id>"
)]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    target_team_id: Json<PalformDatabaseID<IDTeam>>,
    token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    // Membership of current team is already verified, we just need to check the target team
    TeamsRBACManager::from(token.token)
        .require_in_request(
            db.inner(),
            target_team_id.0,
            org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    FormManager::change_form_team(db.inner(), form_id, target_team_id.0)
        .await
        .map_internal_error()?;

    Ok(())
}
