use std::collections::{BTreeMap, HashMap};

use chrono::Utc;
use palform_entities::{legal_agreement, prelude::*, sea_orm_active_enums::LegalAgreementTypeEnum};
use palform_tsid::{
    resources::{IDLegalAgreement, IDOrganisation},
    tsid::PalformDatabaseID,
};
use printpdf::{GeneratePdfOptions, PdfDocument, PdfSaveOptions};
use s3::error::S3Error;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use thiserror::Error;

use crate::{
    actix_util::from_org_id::FromOrgIdTrait,
    api_entities::legal_agreement::APILegalAgreement,
    entity_managers::legal_agreements::agreement_types::{
        all_legal_agreement_types, LegalAgreementType, LegalAgreementTypeDataProcessing,
    },
    palform_s3::{buckets::S3BucketLegalAgreements, client::PalformS3Client},
};

pub struct LegalAgreementsManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for LegalAgreementsManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }
}

#[derive(Debug, Error)]
pub enum LegalAgreementsError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("{0}")]
    S3Error(#[from] S3Error),
    #[error("Creating a PDF: {0}")]
    PDFGeneration(String),
    #[error("The variable {0} was missing.")]
    MissingVariable(String),
}

impl LegalAgreementsManager {
    fn agreement_path(&self, agreement_id: &PalformDatabaseID<IDLegalAgreement>) -> String {
        format!("/{}/{}.pdf", self.org_id, agreement_id)
    }

    async fn signed_url(
        &self,
        s3: &PalformS3Client<S3BucketLegalAgreements>,
        agreement_id: &PalformDatabaseID<IDLegalAgreement>,
    ) -> Result<String, S3Error> {
        let url = s3
            .bucket
            .presign_get(self.agreement_path(&agreement_id), 60 * 60, None)
            .await?;
        Ok(url)
    }

    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketLegalAgreements>,
    ) -> Result<Vec<APILegalAgreement>, LegalAgreementsError> {
        let agreements = LegalAgreement::find()
            .filter(legal_agreement::Column::OrganisationId.eq(self.org_id))
            .order_by_desc(legal_agreement::Column::CreatedAt)
            .all(conn)
            .await?;

        let mut api_agreements: Vec<APILegalAgreement> = Vec::new();
        for agreement in agreements {
            let url = self.signed_url(s3, &agreement.id).await?;
            api_agreements.push(APILegalAgreement::from(agreement, url));
        }

        Ok(api_agreements)
    }

    pub fn all_agreement_types() -> HashMap<String, String> {
        let mut agreement_types_labels = HashMap::new();
        for agreement_types in all_legal_agreement_types() {
            let raw_id_value = serde_json::to_value(agreement_types.agreement_type()).unwrap();
            let raw_id_value = raw_id_value.as_str().unwrap().to_owned();
            agreement_types_labels.insert(raw_id_value, agreement_types.human_name());
        }
        agreement_types_labels
    }

    pub fn agreement_type_struct<'a>(
        agreement_type: &LegalAgreementTypeEnum,
    ) -> impl LegalAgreementType<'a> {
        match agreement_type {
            LegalAgreementTypeEnum::DataProcessing => LegalAgreementTypeDataProcessing,
        }
    }

    fn standard_variable_values<'a>() -> HashMap<String, String> {
        let mut h = HashMap::new();
        let today_date = Utc::now().format("%d/%m/%Y").to_string();
        h.insert("TODAY_DATE".to_owned(), today_date);
        h
    }

    pub async fn generate<'a, T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketLegalAgreements>,
        agreement_type: &LegalAgreementTypeEnum,
        variable_values: &HashMap<String, String>,
    ) -> Result<APILegalAgreement, LegalAgreementsError> {
        let agreement_type_struct = Self::agreement_type_struct(&agreement_type);
        let mut agreement_html = agreement_type_struct.raw_html().to_string();
        let mut variable_values_with_standard = Self::standard_variable_values();
        variable_values_with_standard.extend(variable_values.to_owned());

        for variable in agreement_type_struct.variables() {
            let value = variable_values_with_standard
                .get(&variable.id)
                .ok_or(LegalAgreementsError::MissingVariable(variable.id.clone()))?;

            let pattern = format!("{{{{{}}}}}", variable.id);
            agreement_html = agreement_html.replace(&pattern, value.as_str());
        }

        let pdf_options = GeneratePdfOptions {
            // A4 size
            page_width: Some(210.0),
            page_height: Some(297.0),
            margin_top: Some(5.0),
            margin_left: Some(5.0),
            margin_bottom: Some(5.0),
            margin_right: Some(5.0),
            ..Default::default()
        };

        let mut warnings = Vec::new();
        let empty_btree = BTreeMap::new();
        let pdf_bytes = PdfDocument::from_html(
            &agreement_html,
            &empty_btree,
            &empty_btree,
            &pdf_options,
            &mut warnings,
        )
        .map_err(LegalAgreementsError::PDFGeneration)?;

        let mut pdf_data = Vec::new();
        pdf_bytes.save_writer(&mut pdf_data, &PdfSaveOptions::default(), &mut warnings);

        let new_id = PalformDatabaseID::<IDLegalAgreement>::random();
        let new_agreement = legal_agreement::ActiveModel {
            id: Set(new_id),
            organisation_id: Set(Some(self.org_id)),
            agreement_type: Set(agreement_type.to_owned()),
            ..Default::default()
        };

        let new_model = new_agreement.insert(conn).await?;

        s3.bucket
            .put_object_stream_with_content_type(
                &mut pdf_data.as_ref(),
                self.agreement_path(&new_id),
                "application/pdf",
            )
            .await?;

        let signed_url = self.signed_url(s3, &new_id).await?;

        Ok(APILegalAgreement::from(new_model, signed_url))
    }

    pub async fn verify_agreement_org<T: ConnectionTrait>(
        &self,
        conn: &T,
        agreement_id: &PalformDatabaseID<IDLegalAgreement>,
    ) -> Result<bool, LegalAgreementsError> {
        let count = LegalAgreement::find_by_id(agreement_id.to_owned())
            .filter(legal_agreement::Column::OrganisationId.eq(self.org_id))
            .count(conn)
            .await?;
        Ok(count == 1)
    }

    pub async fn delete<T: ConnectionTrait>(
        &self,
        conn: &T,
        s3: &PalformS3Client<S3BucketLegalAgreements>,
        agreement_id: &PalformDatabaseID<IDLegalAgreement>,
    ) -> Result<(), LegalAgreementsError> {
        LegalAgreement::delete_by_id(agreement_id.to_owned())
            .exec(conn)
            .await?;

        s3.bucket
            .delete_object(self.agreement_path(agreement_id))
            .await?;

        Ok(())
    }
}
