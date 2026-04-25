use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    config::Config,
    entity_managers::orgs::OrganisationManager,
    mail::client::PalformMailClient,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.delete")]
pub async fn organisations_delete(
    path: Path<OrganisationsDeletePath>,
    token: APITokenOrgAdmin,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    db: Data<DatabaseConnection>,
    config: Data<Config>,
    mail: Data<PalformMailClient>,
) -> Result<(), APIError> {
    OrganisationManager::send_staff_deletion_request(path.org_id, config.as_ref(), mail.as_ref())
        .await
        .map_err(|e| APIError::report_internal_error("send org deletion request", e))?;

    audit
        .log_event_with_note(
            db.as_ref(),
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some("Requested deletion of organisation".to_string()),
        )
        .await
        .map_internal_error()?;

    Ok(())
}
