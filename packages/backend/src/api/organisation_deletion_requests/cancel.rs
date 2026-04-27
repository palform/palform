use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, OrganisationDeletionRequestStatusEnum,
};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationDeletionRequest},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{rbac::requests::APITokenOrgAdmin, tokens::APIAuthTokenSource},
    entity_managers::organisation_deletion_request::OrganisationDeletionRequestManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeletionRequestsCancelPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    request_id: PalformDatabaseID<IDOrganisationDeletionRequest>,
}

#[api_operation(
    tag = "Organisation Deletion Requests",
    operation_id = "organisation_deletion_requests.cancel"
)]
pub async fn organisation_deletion_requests_cancel(
    path: Path<OrganisationsDeletionRequestsCancelPath>,
    token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    org_deletion_request_manager: FromOrgId<OrganisationDeletionRequestManager>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if !org_deletion_request_manager
        .verify_request_org_and_status(
            &txn,
            path.request_id,
            Some(OrganisationDeletionRequestStatusEnum::GracePeriod),
        )
        .await
        .map_internal_error()?
    {
        return Err(APIError::NotFound);
    }

    org_deletion_request_manager
        .cancel(&txn, path.request_id)
        .await
        .map_internal_error()?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some(format!(
                "Cancelled deletion request with ID {}",
                path.request_id
            )),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
