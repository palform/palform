use chrono::{Duration, Utc};
use palform_client_common::errors::error::{APIError, APIInternalError};
use palform_entities::{
    organisation_deletion_request, prelude::*,
    sea_orm_active_enums::OrganisationDeletionRequestStatusEnum,
};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation, IDOrganisationDeletionRequest},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use thiserror::Error;

use crate::{
    actix_util::from_org_id::FromOrgIdTrait, billing::manager::CancelPlanRequestReason,
    config::Config,
};

pub struct OrganisationDeletionRequestManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for OrganisationDeletionRequestManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        OrganisationDeletionRequestManager { org_id }
    }
}

#[derive(Error, Debug)]
pub enum OrgDeletionError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[cfg(feature = "saas")]
    #[error("{0}")]
    BillingError(#[from] crate::billing::error::BillingError),
    #[error("Request not found")]
    RequestNotFoundError,
    #[error("Please cancel your active subscription first")]
    ActiveSubscriptionError,
    #[error("Manager was not constructed for request org")]
    OrgIdMismatchError,
}

impl From<OrgDeletionError> for APIError {
    fn from(value: OrgDeletionError) -> Self {
        match value {
            OrgDeletionError::DBError(e) => e.to_internal_error(),
            OrgDeletionError::BillingError(e) => Self::report_internal_error("billing", e),
            OrgDeletionError::RequestNotFoundError => Self::BadRequest(value.to_string()),
            OrgDeletionError::ActiveSubscriptionError => Self::BadRequest(value.to_string()),
            OrgDeletionError::OrgIdMismatchError => {
                Self::report_internal_error("org deletion", value)
            }
        }
    }
}

impl OrganisationDeletionRequestManager {
    pub async fn create_request<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        include_user: bool,
        reason: CancelPlanRequestReason,
        #[cfg(feature = "saas")] stripe: &stripe::Client,
    ) -> Result<PalformDatabaseID<IDOrganisationDeletionRequest>, OrgDeletionError> {
        #[cfg(feature = "saas")]
        {
            let manager = crate::billing::manager::BillingManager::new(stripe);
            let active_subscriptions = manager.get_org_plans(conn, self.org_id).await?;
            for subscription in &active_subscriptions {
                if !subscription.canceling_at_end {
                    return Err(OrgDeletionError::ActiveSubscriptionError);
                }
            }
        }

        let id = PalformDatabaseID::<IDOrganisationDeletionRequest>::random();
        let new_request = organisation_deletion_request::ActiveModel {
            id: Set(id),
            organisation_id: Set(self.org_id),
            user_id: Set(user_id),
            include_user: Set(include_user),
            reason: Set(reason.to_string()),
            status: Set(OrganisationDeletionRequestStatusEnum::GracePeriod),
            ..Default::default()
        };

        new_request.insert(conn).await?;
        Ok(id)
    }

    pub async fn list_pending<T: ConnectionTrait>(
        conn: &T,
        config: &Config,
    ) -> Result<Vec<organisation_deletion_request::Model>, DbErr> {
        let created_before = Utc::now()
            - Duration::hours(i64::from(config.organisation_deletion_grace_period_hours));

        OrganisationDeletionRequest::find()
            .filter(
                organisation_deletion_request::Column::Status
                    .eq(OrganisationDeletionRequestStatusEnum::GracePeriod),
            )
            .filter(organisation_deletion_request::Column::CreatedAt.lte(created_before))
            .order_by_asc(organisation_deletion_request::Column::CreatedAt)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn verify_request_org_and_status<T: ConnectionTrait>(
        &self,
        conn: &T,
        request_id: PalformDatabaseID<IDOrganisationDeletionRequest>,
        status: Option<OrganisationDeletionRequestStatusEnum>,
    ) -> Result<bool, DbErr> {
        let (actual_org_id, actual_status): (
            PalformDatabaseID<IDOrganisation>,
            OrganisationDeletionRequestStatusEnum,
        ) = OrganisationDeletionRequest::find_by_id(request_id)
            .select_only()
            .column(organisation_deletion_request::Column::OrganisationId)
            .column(organisation_deletion_request::Column::Status)
            .into_tuple()
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("not found".to_string()))?;

        if actual_org_id != self.org_id {
            return Ok(false);
        }

        Ok(status.is_none_or(|s| s == actual_status))
    }

    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Vec<organisation_deletion_request::Model>, DbErr> {
        OrganisationDeletionRequest::find()
            .filter(organisation_deletion_request::Column::OrganisationId.eq(self.org_id))
            .order_by_desc(organisation_deletion_request::Column::CreatedAt)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get<T: ConnectionTrait>(
        &self,
        conn: &T,
        id: PalformDatabaseID<IDOrganisationDeletionRequest>,
    ) -> Result<Option<organisation_deletion_request::Model>, DbErr> {
        OrganisationDeletionRequest::find_by_id(id)
            .filter(organisation_deletion_request::Column::OrganisationId.eq(self.org_id))
            .into_model()
            .one(conn)
            .await
    }

    pub async fn cancel<T: ConnectionTrait>(
        &self,
        conn: &T,
        request_id: PalformDatabaseID<IDOrganisationDeletionRequest>,
    ) -> Result<(), DbErr> {
        let updated_request = organisation_deletion_request::ActiveModel {
            id: Set(request_id),
            status: Set(OrganisationDeletionRequestStatusEnum::Cancelled),
            ..Default::default()
        };
        updated_request.update(conn).await?;
        Ok(())
    }
    pub async fn execute_request<T: ConnectionTrait>(
        &self,
        conn: &T,
        request: &organisation_deletion_request::Model,
        #[cfg(feature = "saas")] stripe: &stripe::Client,
    ) -> Result<(), OrgDeletionError> {
        if request.organisation_id != self.org_id {
            return Err(OrgDeletionError::OrgIdMismatchError);
        }

        #[cfg(feature = "saas")]
        {
            let manager = crate::billing::manager::BillingManager::new(stripe);
            manager
                .delete_customer(conn, request.organisation_id)
                .await?;
        }

        Organisation::delete_by_id(request.organisation_id)
            .exec(conn)
            .await?;

        if request.include_user {
            AdminUser::delete_by_id(request.user_id).exec(conn).await?;
        }

        let updated_request = organisation_deletion_request::ActiveModel {
            id: Set(request.id),
            status: Set(OrganisationDeletionRequestStatusEnum::Deleted),
            ..Default::default()
        };
        updated_request.update(conn).await?;
        Ok(())
    }
}
