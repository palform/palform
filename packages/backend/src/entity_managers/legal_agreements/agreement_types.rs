use palform_entities::sea_orm_active_enums::LegalAgreementTypeEnum;

use crate::api_entities::legal_agreement::APILegalAgreementVariable;

pub trait LegalAgreementType<'a> {
    fn agreement_type(&self) -> LegalAgreementTypeEnum;
    fn human_name(&self) -> String;
    fn variables(&self) -> Vec<APILegalAgreementVariable>;
    fn raw_html(&self) -> &'a str;
}

pub struct LegalAgreementTypeDataProcessing;
impl<'a> LegalAgreementType<'a> for LegalAgreementTypeDataProcessing {
    fn agreement_type(&self) -> LegalAgreementTypeEnum {
        LegalAgreementTypeEnum::DataProcessing
    }
    fn human_name(&self) -> String {
        "Data Processing Agreement".to_string()
    }
    fn variables(&self) -> Vec<APILegalAgreementVariable> {
        vec![
            APILegalAgreementVariable {
                id: "TODAY_DATE".to_string(),
                name: "Today's date".to_string(),
                is_standard: true,
            },
            APILegalAgreementVariable {
                id: "CONTROLLER_NAME".to_string(),
                name: "Organisation name".to_string(),
                is_standard: false,
            },
            APILegalAgreementVariable {
                id: "CONTROLLER_ADDRESS".to_string(),
                name: "Organisation address".to_string(),
                is_standard: false,
            },
            APILegalAgreementVariable {
                id: "CONTROLLER_PERSON_NAME".to_string(),
                name: "Representative person name".to_string(),
                is_standard: false,
            },
            APILegalAgreementVariable {
                id: "CONTROLLER_PERSON_POSITION".to_string(),
                name: "Representative person position".to_string(),
                is_standard: false,
            },
            APILegalAgreementVariable {
                id: "CONTROLLER_PERSON_EMAIL_ADDRESS".to_string(),
                name: "Representative person email address".to_string(),
                is_standard: false,
            },
        ]
    }
    fn raw_html(&self) -> &'a str {
        include_str!("../../../assets/legal_agreements/data_processing_agreement.html")
    }
}

pub(super) fn all_legal_agreement_types<'a>() -> Vec<impl LegalAgreementType<'a>> {
    vec![LegalAgreementTypeDataProcessing]
}
