use actix_web::web::{Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_entities::sea_orm_active_enums::LegalAgreementTypeEnum;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    api_entities::legal_agreement::APILegalAgreementVariable,
    entity_managers::legal_agreements::{
        agreement_types::LegalAgreementType, manager::LegalAgreementsManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct LegalAgreementListPath {
    #[allow(unused)]
    org_id: PalformDatabaseID<IDOrganisation>,
    agreement_type: LegalAgreementTypeEnum,
}

#[api_operation(
    tag = "Legal Agreements",
    operation_id = "legal_agreements.list_variables"
)]
pub async fn legal_agreements_variables_list(
    path: Path<LegalAgreementListPath>,
) -> Json<Vec<APILegalAgreementVariable>> {
    let agreement_type_struct = LegalAgreementsManager::agreement_type_struct(&path.agreement_type);
    Json(
        agreement_type_struct
            .variables()
            .iter()
            .filter(|v| !v.is_standard)
            .cloned()
            .collect(),
    )
}
