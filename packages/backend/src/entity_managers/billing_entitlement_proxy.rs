use palform_client_common::errors::error::{APIError, APIErrorWithStatus};
use palform_tsid::{
    resources::{IDOrganisation, IDUnknown},
    tsid::PalformDatabaseID,
};
use sea_orm::{ConnectionTrait, DbErr};

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    rocket_util::from_org_id::FromOrgIdTrait,
};

pub trait BillingEntitlementCountTrait {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<u64, DbErr>;
}

pub trait BillingEntitlementContextualCountTrait {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        context_resource_id: PalformDatabaseID<IDUnknown>,
    ) -> Result<u64, DbErr>;
}

pub struct BillingEntitlementManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for BillingEntitlementManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }
}

impl BillingEntitlementManager {
    #[cfg(feature = "saas")]
    fn get_internal_manager(
        &self,
    ) -> crate::billing::entitlement::INTERNALBillingEntitlementManager {
        crate::billing::entitlement::INTERNALBillingEntitlementManager::new(self.org_id)
    }

    pub async fn check_entitlement<T: ConnectionTrait>(
        &self,
        conn: &T,
        entitlement_req: APIEntitlementRequest,
    ) -> Result<(), APIErrorWithStatus> {
        #[cfg(feature = "saas")]
        {
            let m = self.get_internal_manager();
            let allowed = m
                .check_entitlement(conn, entitlement_req.clone())
                .await
                .map_err(|e| APIError::report_internal_error("check billing entitlement", e))?;
            if !allowed {
                Err(APIError::SubscriptionLimit(entitlement_req.to_string()).into())
            } else {
                Ok(())
            }
        }

        #[cfg(not(feature = "saas"))]
        Ok(())
    }

    pub async fn create_initial_entitlement<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<(), DbErr> {
        #[cfg(feature = "saas")]
        {
            let m = self.get_internal_manager();
            m.create_initial_entitlement(conn).await
        }

        #[cfg(not(feature = "saas"))]
        Ok(())
    }
}
