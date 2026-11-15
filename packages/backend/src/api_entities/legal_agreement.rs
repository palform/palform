use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::{legal_agreement, sea_orm_active_enums::LegalAgreementTypeEnum};
use palform_tsid::{resources::IDLegalAgreement, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct APILegalAgreement {
    pub id: PalformDatabaseID<IDLegalAgreement>,
    pub agreement_type: LegalAgreementTypeEnum,
    pub created_at: DateTime<Utc>,
    pub document_url: String,
}

impl APILegalAgreement {
    pub fn from(model: legal_agreement::Model, url: String) -> APILegalAgreement {
        APILegalAgreement {
            id: model.id,
            agreement_type: model.agreement_type,
            created_at: model.created_at.to_utc(),
            document_url: url,
        }
    }
}

#[derive(Serialize, JsonSchema, ApiComponent, Clone)]
pub struct APILegalAgreementVariable {
    pub id: String,
    pub name: String,
    pub is_standard: bool,
}
