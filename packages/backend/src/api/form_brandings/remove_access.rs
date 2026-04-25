use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationMemberRoleEnum,
};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainBranding, manager::AuditManager},
    auth::{
        rbac::{requests::APITokenTeamAdminFromTeam, teams_manager::TeamsRBACManager},
        tokens::APIAuthTokenSource,
    },
    entity_managers::form_brandings::FormBrandingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct DeleteAccessRequest {
    for_team_id: PalformDatabaseID<IDTeam>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormBrandingDeleteAccessPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    #[allow(unused)]
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
}

#[api_operation(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.delete_access"
)]
pub async fn form_brandings_remove_access(
    path: Path<FormBrandingDeleteAccessPath>,
    data: Json<DeleteAccessRequest>,
    token: APITokenTeamAdminFromTeam,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainBranding>>,
) -> Result<(), APIError> {
    TeamsRBACManager::from(token.token.clone())
        .require_in_request(
            db.as_ref(),
            data.for_team_id,
            path.org_id,
            OrganisationMemberRoleEnum::Admin,
        )
        .await?;

    FormBrandingManager::remove_access(db.as_ref(), path.branding_id, data.for_team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_id_chain_and_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Branding,
            Some(path.branding_id.into_unknown()),
            Some(format!("Removed access for team {}", data.for_team_id)),
            Some(IDChainBranding::new(path.team_id)),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
