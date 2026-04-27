use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::OrganisationDeletionRequestStatusEnum;
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationDeletionRequest},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId, auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_deletion_request::OrganisationDeletionRequestManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeletionRequestsSkipPath {
    #[allow(dead_code)]
    org_id: PalformDatabaseID<IDOrganisation>,
    request_id: PalformDatabaseID<IDOrganisationDeletionRequest>,
}

#[api_operation(
    tag = "Organisation Deletion Requests",
    operation_id = "organisation_deletion_requests.skip"
)]
pub async fn organisation_deletion_requests_skip(
    path: Path<OrganisationsDeletionRequestsSkipPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    org_deletion_request_manager: FromOrgId<OrganisationDeletionRequestManager>,
    stripe: Data<stripe::Client>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let request = org_deletion_request_manager
        .get(&txn, path.request_id)
        .await
        .map_internal_error()?
        .ok_or(APIError::NotFound)?;
    if request.status != OrganisationDeletionRequestStatusEnum::GracePeriod {
        return Err(APIError::BadRequest(
            "Deletion request must be within the grace period".to_string(),
        ));
    }

    org_deletion_request_manager
        .execute_request(&txn, &request, &stripe)
        .await?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
