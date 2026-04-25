use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationAuthTeamMapping},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationAuthTeamMappingsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    mapping_id: PalformDatabaseID<IDOrganisationAuthTeamMapping>,
}

#[api_operation(
    tag = "Organisation Authentication Team Mappings",
    operation_id = "organisation.auth_config.mappings.delete"
)]
pub async fn organisation_auth_team_mappings_delete(
    path: Path<OrganisationAuthTeamMappingsDeletePath>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<(), APIError> {
    if !m
        .verify_mapping_org(db.as_ref(), path.mapping_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    m.delete(db.as_ref(), path.mapping_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationAuthConfig,
            Some(path.org_id.into_unknown()),
            Some("Deleted team mapping".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
