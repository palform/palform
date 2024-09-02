use palform_client_common::errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::{IDOrganisation, IDOrganisationAuthTeamMapping}, tsid::PalformDatabaseID};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api_entities::{
        billing::entitlement::APIEntitlementRequest,
        organisation_auth_team_mapping::APIOrganisationAuthTeamMappingRequest,
    },
    audit::AuditManager,
    auth::rbac::requests::APITokenOrgAdmin,
    auth::tokens::APIAuthTokenSource,
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
        organisation_teams::OrganisationTeamsManager,
    },
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Authentication Team Mappings",
    operation_id = "organisation.auth_config.mappings.create"
)]
#[post("/users/me/orgs/<org_id>/auth_config/mappings", data = "<request>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    request: Json<APIOrganisationAuthTeamMappingRequest>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    billing: FromOrgId<BillingEntitlementManager>,
    m: FromOrgId<OrganisationAuthTeamMappingsManager>,
) -> Result<Json<PalformDatabaseID<IDOrganisationAuthTeamMapping>>, APIErrorWithStatus> {
    billing
        .check_entitlement(db.inner(), APIEntitlementRequest::OIDC)
        .await?;

    if !OrganisationTeamsManager::verify_team_org(db.inner(), request.team_id, org_id)
        .await
        .map_internal_error()?
    {
        return Err(APIError::BadRequest("Team not found in org".to_string()).into());
    }

    let new_id = m.create(db.inner(), request.0).await.map_internal_error()?;

    audit
        .log_event_with_note(
            db.inner(),
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::OrganisationAuthConfig,
            Some(org_id.into_unknown()),
            Some("Created team mapping".to_string()),
        )
        .await
        .map_internal_error()?;
    Ok(Json(new_id))
}
