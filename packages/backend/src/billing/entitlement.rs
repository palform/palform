use palform_entities::{organisation_feature_entitlement, prelude::*};
use palform_tsid::{
    resources::{IDOrganisation, IDUnknown},
    tsid::PalformDatabaseID,
};
use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set};

use crate::{
    api_entities::billing::entitlement::{APIEntitlementInfo, APIEntitlementRequest},
    entity_managers::{
        billing_entitlement_proxy::{
            BillingEntitlementContextualCountTrait, BillingEntitlementCountTrait,
        },
        form_brandings::FormBrandingManager,
        forms::FormManager,
        organisation_members::OrganisationMembersManager,
        organisation_teams::OrganisationTeamsManager,
        questions::QuestionManager,
    },
};

use super::error::BillingError;

/// Don't use this struct directly, use
/// [BillingEntitlementManager](crate::entity_managers::billing_entitlement::BillingEntitlementManager) instead
pub struct INTERNALBillingEntitlementManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl INTERNALBillingEntitlementManager {
    pub fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }

    async fn get_org_entitlement<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<organisation_feature_entitlement::Model, DbErr> {
        OrganisationFeatureEntitlement::find_by_id(self.org_id.clone())
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Organisation did not exist".to_string(),
            ))
    }

    pub async fn get_entitlement_info<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<APIEntitlementInfo, DbErr> {
        OrganisationFeatureEntitlement::find_by_id(self.org_id.clone())
            .into_model()
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Organisation did not exist".to_string(),
            ))
    }

    async fn numeric_check<C: ConnectionTrait, M: BillingEntitlementCountTrait>(
        &self,
        conn: &C,
        entitlement: Option<i32>,
    ) -> Result<bool, DbErr> {
        if let Some(entitlement_val) = entitlement {
            let count = M::billing_count(conn, self.org_id.clone()).await?;
            let count = count as i32;
            // add 1 since we're checking if _adding_ a new record would be a valid operation
            Ok(count + 1 <= entitlement_val)
        } else {
            // None means infinite is allowed
            Ok(true)
        }
    }

    async fn contextual_numeric_check<
        C: ConnectionTrait,
        M: BillingEntitlementContextualCountTrait,
    >(
        &self,
        conn: &C,
        context_resource_id: PalformDatabaseID<IDUnknown>,
        entitlement: Option<i32>,
    ) -> Result<bool, DbErr> {
        if let Some(entitlement_val) = entitlement {
            let count = M::billing_count(conn, self.org_id.clone(), context_resource_id).await?;
            let count = count as i32;
            // add 1 since we're checking if _adding_ a new record would be a valid operation
            Ok(count + 1 <= entitlement_val)
        } else {
            // None means infinite is allowed
            Ok(true)
        }
    }

    pub async fn check_entitlement<T: ConnectionTrait>(
        &self,
        conn: &T,
        entitlement_req: APIEntitlementRequest,
    ) -> Result<bool, BillingError> {
        let oe = self.get_org_entitlement(conn).await?;
        Ok(match entitlement_req {
            APIEntitlementRequest::UserCount => {
                self.numeric_check::<_, OrganisationMembersManager>(conn, oe.user_count)
                    .await?
            }
            APIEntitlementRequest::ResponseCount => {
                // this is checked separately, see the API handler for new submission
                true
            }
            APIEntitlementRequest::QuestionPerFormCount(form_id) => {
                self.contextual_numeric_check::<_, QuestionManager>(
                    conn,
                    form_id.into_unknown(),
                    oe.question_per_form_count,
                )
                .await?
            }
            APIEntitlementRequest::FormCount => {
                self.numeric_check::<_, FormManager>(conn, oe.form_count)
                    .await?
            }
            APIEntitlementRequest::TeamCount => {
                self.numeric_check::<_, OrganisationTeamsManager>(conn, oe.team_count)
                    .await?
            }
            APIEntitlementRequest::BrandingCount => {
                self.numeric_check::<_, FormBrandingManager>(conn, oe.branding_count)
                    .await?
            }
            APIEntitlementRequest::Export => oe.export,
            APIEntitlementRequest::ImportKeys => oe.import_keys,
            APIEntitlementRequest::Subdomain => oe.subdomain,
            APIEntitlementRequest::CryptoDetails => oe.crypto_details,
            APIEntitlementRequest::Audit => oe.audit,
            APIEntitlementRequest::PrioritySupport => oe.priority_support,
            APIEntitlementRequest::OIDC => oe.oidc,
            APIEntitlementRequest::SubmissionAutoDelete => oe.submission_auto_delete,
            APIEntitlementRequest::FormCaptcha => oe.form_captcha,
        })
    }

    pub async fn create_initial_entitlement<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<(), DbErr> {
        let mut new_entitlement = APIEntitlementInfo::default().to_active_model();
        new_entitlement.organisation_id = Set(self.org_id.clone());
        new_entitlement.insert(conn).await?;
        Ok(())
    }
}
