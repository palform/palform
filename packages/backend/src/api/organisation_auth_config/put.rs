use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{put, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api_entities::{
        billing::entitlement::APIEntitlementRequest,
        organisation_auth_config::APIOrganisationAuthConfig,
    },
    audit::AuditManager,
    auth::rbac::requests::APITokenOrgAdmin,
    auth::tokens::APIAuthTokenSource,
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        organisation_auth_config::OrganisationAuthConfigManager,
    },
    rocket_util::from_org_id::FromOrgId,
};

#[openapi(
    tag = "Organisation Authentication Configuration",
    operation_id = "organisation.auth_config.put"
)]
#[put("/users/me/orgs/<org_id>/auth_config", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<Option<APIOrganisationAuthConfig>>,
    token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
    audit: FromOrgId<AuditManager>,
    m: FromOrgId<OrganisationAuthConfigManager>,
    billing_m: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    billing_m
        .check_entitlement(&txn, APIEntitlementRequest::OIDC)
        .await?;

    if let Some(data) = data.0.clone() {
        m.set(&txn, data).await.map_internal_error()?;
    } else {
        m.delete(&txn).await.map_internal_error()?;
    }

    audit
        .log_event(
            &txn,
            token.get_user_id(),
            if data.is_some() {
                AuditLogVerbEnum::Update
            } else {
                AuditLogVerbEnum::Delete
            },
            AuditLogTargetResourceEnum::OrganisationAuthConfig,
            Some(org_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
