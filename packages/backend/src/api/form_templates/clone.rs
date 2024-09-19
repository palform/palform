use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    api_entities::form::APIForm, auth::rbac::{requests::APITokenOrgViewer, teams_manager::TeamsRBACManager}, entity_managers::form_templates::FormTemplatesManager
};

#[derive(Deserialize, JsonSchema)]
pub struct CloneFormTemplateRequest {
    into_team: PalformDatabaseID<IDTeam>,
}

#[openapi(tag = "Form Templates", operation_id = "form_templates.clone")]
#[post("/users/me/orgs/<org_id>/templates/<template_id>/clone", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    template_id: PalformDatabaseID<IDForm>,
    data: Json<CloneFormTemplateRequest>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIForm>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    TeamsRBACManager::from(token.token)
        .require_in_request(
            &txn,
            data.into_team,
            org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    let new_form = FormTemplatesManager::clone(&txn, template_id, data.into_team)
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;

    Ok(Json(new_form))
}
