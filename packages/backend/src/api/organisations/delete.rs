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
    api_entities::org::APIOrganisationManifest,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    billing::manager::CancelPlanRequestReason,
    config::Config,
    entity_managers::{
        organisation_deletion_request::OrganisationDeletionRequestManager,
        orgs::OrganisationManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeleteRequest {
    dry_run: bool,
    include_own_account: bool,
    reason: CancelPlanRequestReason,
}

#[api_operation(tag = "Organisations", operation_id = "orgs.delete")]
pub async fn organisations_delete(
    path: Path<OrganisationsDeletePath>,
    data: Json<OrganisationsDeleteRequest>,
    token: APITokenOrgAdmin,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
    db: Data<DatabaseConnection>,
    org_deletion_request_manager: FromOrgId<OrganisationDeletionRequestManager>,
    stripe: Data<stripe::Client>,
    config: Data<Config>,
) -> Result<Json<Option<APIOrganisationManifest>>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if data.dry_run {
        let manifest = OrganisationManager::generate_manifest(&txn, path.org_id, &stripe)
            .await
            .map_err(|e| APIError::report_internal_error("generating manifest", e))?;

        // We don't expect any data to have been written
        txn.rollback().await.map_internal_error()?;
        return Ok(Json(Some(manifest)));
    }

    org_deletion_request_manager
        .create_request(
            &txn,
            token.get_user_id(),
            data.include_own_account,
            data.reason.clone(),
            &stripe,
        )
        .await?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Delete,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some(format!(
                "Requested deletion of organisation. Will take place in {} hours unless cancelled.",
                config.organisation_deletion_grace_period_hours
            )),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(None))
}
