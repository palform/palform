use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::legal_agreement::APILegalAgreement,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::legal_agreements::manager::LegalAgreementsManager,
    palform_s3::{buckets::S3BucketLegalAgreements, client::PalformS3Client},
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Legal Agreements", operation_id = "legal_agreements.list")]
pub async fn legal_agreements_list(
    _path: Path<LegalAgreementListPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    s3: Data<PalformS3Client<S3BucketLegalAgreements>>,
    m: FromOrgId<LegalAgreementsManager>,
) -> Result<Json<Vec<APILegalAgreement>>, APIError> {
    let agreements = m
        .list(db.as_ref(), s3.as_ref())
        .await
        .map_err(|e| APIError::report_internal_error("listing legal agreements", e))?;
    Ok(Json(agreements))
}
