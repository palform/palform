use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::form_brandings::APIFormBrandingRequest,
    audit::{id_chains::IDChainBranding, manager::AuditManager},
    auth::{rbac::requests::APITokenTeamEditorFromTeam, tokens::APIAuthTokenSource},
    entity_managers::form_brandings::FormBrandingManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct FormBrandingsCreatePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    team_id: PalformDatabaseID<IDTeam>,
}

#[api_operation(
    tag = "Form Brandings",
    operation_id = "organisation.team.branding.create"
)]
pub async fn form_brandings_create(
    path: Path<FormBrandingsCreatePath>,
    data: Json<APIFormBrandingRequest>,
    token: APITokenTeamEditorFromTeam,
    audit: FromOrgId<AuditManager<IDChainBranding>>,
    db: Data<DatabaseConnection>,
) -> Result<Json<PalformDatabaseID<IDFormBranding>>, APIError> {
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

    FormBrandingManager::add_access(&txn, branding_id, path.team_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_id_chain(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Create,
            AuditLogTargetResourceEnum::Branding,
            Some(branding_id.into_unknown()),
            Some(IDChainBranding::new(path.team_id)),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(branding_id))
}
