use std::collections::HashMap;

use actix_web::web::{Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::entity_managers::legal_agreements::manager::LegalAgreementsManager;

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementsListAgreementTypesPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementsListAgreementTypesResponse {
    #[serde(flatten)]
    types: HashMap<String, String>,
}

#[api_operation(
    tag = "Legal Agreements",
    operation_id = "legal_agreements.list_agreement_types"
)]
pub async fn legal_agreements_list_agreement_types(
    _path: Path<LegalAgreementsListAgreementTypesPath>,
) -> Json<LegalAgreementsListAgreementTypesResponse> {
    Json(LegalAgreementsListAgreementTypesResponse {
        types: LegalAgreementsManager::all_agreement_types(),
    })
}
