use actix_web::web::{Data, Json, Path};
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
    api_entities::{
        billing::entitlement::APIEntitlementRequest,
        organisation_auth_team_mapping::APIOrganisationAuthTeamMappingRequest,
    },
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
        organisation_teams::OrganisationTeamsManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationAuthTeamMappingsCreatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Authentication Team Mappings",
    operation_id = "organisation.auth_config.mappings.create"
)]
pub async fn organisation_auth_team_mappings_create(
    path: Path<OrganisationAuthTeamMappingsCreatePath>,
    request: Json<APIOrganisationAuthTeamMappingRequest>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    billing: FromOrgId<BillingEntitlementManager>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<Json<PalformDatabaseID<IDOrganisationAuthTeamMapping>>, APIError> {
    billing
        .check_entitlement(db.as_ref(), APIEntitlementRequest::OIDC)
        .await?;

    if !OrganisationTeamsManager::verify_team_org(db.as_ref(), request.team_id, path.org_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::BadRequest("Team not found in org".to_string()).into());
    }

    let new_id = m
        .create(db.as_ref(), request.0)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationAuthConfig,
            Some(path.org_id.into_unknown()),
            Some("Created team mapping".to_string()),
        )
        .await
        .map_internal_error()?;
    Ok(Json(new_id))
}
