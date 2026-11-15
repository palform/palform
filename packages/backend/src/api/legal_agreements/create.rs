use std::collections::HashMap;

use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalError, APIInternalErrorResult};
use palform_entities::sea_orm_active_enums::{
    AuditLogTargetResourceEnum, AuditLogVerbEnum, LegalAgreementTypeEnum,
};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::legal_agreement::APILegalAgreement,
    audit::{id_chains::IDChainEmpty, manager::AuditManager},
    auth::{
        rbac::requests::{OrgRoleToken, TeamRoleNone, TokenTargetOrgAdmin},
        tokens::{APIAuthTokenSource, APIAuthTokenSourcePersonal},
    },
    entity_managers::legal_agreements::manager::LegalAgreementsManager,
    palform_s3::{buckets::S3BucketLegalAgreements, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementCreatePath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    agreement_type: LegalAgreementTypeEnum,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementCreateRequest {
    values: HashMap<String, String>,
}

#[api_operation(tag = "Legal Agreements", operation_id = "legal_agreements.create")]
pub async fn legal_agreements_create(
    path: Path<LegalAgreementCreatePath>,
    data: Json<LegalAgreementCreateRequest>,
    token: OrgRoleToken<TeamRoleNone, TokenTargetOrgAdmin, APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
    s3: Data<PalformS3Client<S3BucketLegalAgreements>>,
    m: FromOrgId<LegalAgreementsManager>,
    audit: FromOrgId<AuditManager<IDChainEmpty>>,
) -> Result<Json<APILegalAgreement>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let new_agreement = m
        .generate(&txn, s3.as_ref(), &path.agreement_type, &data.values)
        .await
        .map_err(|e| APIError::report_internal_error("listing legal agreements", e))?;

    audit
        .log_event_with_note(
            &txn,
            token.get_user_id(),
            AuditLogVerbEnum::Update,
            AuditLogTargetResourceEnum::Organisation,
            Some(path.org_id.into_unknown()),
            Some(format!("Generated legal agreement")),
        )
        .await
        .map_internal_error()?;

    txn.commit().await.map_internal_error()?;
    Ok(Json(new_agreement))
}
