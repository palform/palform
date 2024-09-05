use palform_client_common::errors::error::APIInternalErrorResult;
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::resources::{IDFormBranding, IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use rocket::{http::Status, post, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::api_entities::form::APIForm;
use crate::audit::AuditManager;
use crate::auth::rbac::{requests::APITokenOrgViewer, teams_manager::TeamsRBACManager};
use crate::auth::tokens::APIAuthTokenSource;
use crate::entity_managers::form_brandings::FormBrandingManager;
use crate::rocket_util::from_org_id::FromOrgId;
use crate::{
    api::error::{APIError, APIInternalError},
    entity_managers::forms::FormManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct CreateFormRequest {
    editor_name: String,
    title: String,
    in_team: PalformDatabaseID<IDTeam>,
    branding_id: Option<PalformDatabaseID<IDFormBranding>>,
    one_question_per_page: bool,
}

#[openapi(tag = "Forms", operation_id = "forms.create")]
#[post("/users/me/orgs/<org_id>/forms", data = "<request>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    request: Json<CreateFormRequest>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
) -> Result<Json<APIForm>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::Serializable),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            &txn,
            request.in_team,
            org_id,
            OrganisationMemberRoleEnum::Editor,
        )
        .await?;

    if let Some(branding_id) = request.branding_id {
        if !FormBrandingManager::verify_branding_team_allowed(&txn, branding_id, request.in_team)
            .await
            .map_internal_error()?
        {
            return Err(APIError::BadRequest(format!(
                "branding {} not found in team {}",
                branding_id, request.in_team
            ))
            .into());
        }
    }

    let new_form = FormManager::create(
        &txn,
        request.in_team,
        request.editor_name.clone(),
        request.title.clone(),
        request.branding_id,
        request.one_question_per_page,
    )
    .await
    .map_err(|e| e.to_internal_error())?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::Form,
            Some(new_form.id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_err(|e| e.to_internal_error())?;
    Ok(Json(new_form))
}
