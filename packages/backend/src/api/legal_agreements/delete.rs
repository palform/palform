use actix_web::web::{Data, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{AuditLogTargetResourceEnum, AuditLogVerbEnum};
use palform_tsid::{
    resources::{IDLegalAgreement, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        rbac::requests::{APITokenOrgAdmin, OrgRoleToken, TeamRoleNone, TokenTargetOrgAdmin},
        tokens::{APIAuthTokenSource, APIAuthTokenSourcePersonal},
    },
    entity_managers::legal_agreements::manager::LegalAgreementsManager,
    palform_s3::{buckets::S3BucketLegalAgreements, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementsDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    agreement_id: PalformDatabaseID<IDLegalAgreement>,
}

#[api_operation(tag = "Legal Agreements", operation_id = "legal_agreements.delete")]
pub async fn legal_agreements_delete(
    path: Path<LegalAgreementsDeletePath>,
    token: OrgRoleToken<TeamRoleNone, TokenTargetOrgAdmin, APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    s3: Data<PalformS3Client<S3BucketLegalAgreements>>,
    m: FromOrgId<LegalAgreementsManager>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    if !m
        .verify_agreement_org(&txn, &path.agreement_id)
        .await
        .map_err(|e| APIError::report_internal_error("verify agreement org", e))?
    {
        return Err(APIError::NotFound);
    }

    m.delete(&txn, &s3, &path.agreement_id)
        .await
        .map_err(|e| APIError::report_internal_error("delete agreement", e))?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some("Deleted legal agreement".to_string()),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
