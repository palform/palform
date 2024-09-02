use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::{IDOrganisation, IDOrganisationAuthTeamMapping}, tsid::PalformDatabaseID};
use rocket::{delete, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    audit::AuditManager, auth::rbac::requests::APITokenOrgAdmin, auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Authentication Team Mappings",
    operation_id = "organisation.auth_config.mappings.delete"
)]
#[delete("/users/me/orgs/<org_id>/auth_config/mappings/<mapping_id>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    mapping_id: PalformDatabaseID<IDOrganisationAuthTeamMapping>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<(), APIErrorWithStatus> {
    if !m
        .verify_mapping_org(db.inner(), mapping_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound.into());
    }

    m.delete(db.inner(), mapping_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationAuthConfig,
            Some(org_id.into_unknown()),
            Some("Deleted team mapping".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
