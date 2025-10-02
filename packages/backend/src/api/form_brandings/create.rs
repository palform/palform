use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api_entities::form_brandings::APIFormBrandingRequest,
    audit::AuditManager,
    auth::{rbac::requests::APITokenTeamEditorFromTeam, tokens::APIAuthTokenSource},
    entity_managers::form_brandings::FormBrandingManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.create"
)]
#[post("/users/me/orgs/<_org_id>/teams/<team_id>/brandings", data = "<data>")]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    data: Json<APIFormBrandingRequest>,
    token: APITokenTeamEditorFromTeam,
    audit: FromOrgId<AuditManager>,
    db: &State<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDFormBranding>>, APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let branding_id = FormBrandingManager::create(&txn, data.0)
        .await
        .map_internal_error()?;

    FormBrandingManager::add_access(&txn, branding_id, team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Branding,
            Some(branding_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(branding_id))
}
