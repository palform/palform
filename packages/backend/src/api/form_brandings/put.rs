use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::form_brandings::APIFormBrandingRequest,
    audit::{id_chains::IDChainBranding, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamEditorFromTeam, tokens::APIAuthTokenSource},
    entity_managers::form_brandings::FormBrandingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormBrandingsPutPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
    branding_id: PalformDatabaseID<IDFormBranding>,
}

#[api_operation(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.put"
)]
pub async fn form_brandings_put(
    path: Path<FormBrandingsPutPath>,
    data: Json<APIFormBrandingRequest>,
    token: APITokenTeamEditorFromTeam,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainBranding>>,
) -> Result<(), APIError> {
    FormBrandingManager::verify_branding_team_allowed(db.as_ref(), path.branding_id, path.team_id)
        .await
        .map_internal_error()?;

    FormBrandingManager::update(db.as_ref(), path.branding_id, data.0)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_id_chain(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Branding,
            Some(path.branding_id.into_unknown()),
            Some(IDChainBranding::new(path.team_id)),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
