use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::{
        billing::entitlement::APIEntitlementRequest,
        organisation_auth_config::APIOrganisationAuthConfig,
    },
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        organisation_auth_config::OrganisationAuthConfigManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationAuthConfigPutPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Authentication Configuration",
    operation_id = "organisation.auth_config.put"
)]
pub async fn organisation_auth_config_put(
    path: Path<OrganisationAuthConfigPutPath>,
    data: Json<Option<APIOrganisationAuthConfig>>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    m: FromOrgId<OrganisationAuthConfigManager>,
    billing_m: FromOrgId<BillingEntitlementManager>,
) -> Result<(), APIError> {
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
            Some(path.org_id.into_unknown()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
