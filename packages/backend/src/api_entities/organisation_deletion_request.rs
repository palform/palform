use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::sea_orm_active_enums::OrganisationDeletionRequestStatusEnum;
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation, IDOrganisationDeletionRequest},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use serde::Serialize;

use crate::billing::manager::CancelPlanRequestReason;

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct APIOrganisationDeletionRequest {
    pub id: PalformDatabaseID<IDOrganisationDeletionRequest>,
    pub organisation_id: PalformDatabaseID<IDOrganisation>,
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub include_user: bool,
    pub status: OrganisationDeletionRequestStatusEnum,
    pub reason: CancelPlanRequestReason,
    pub created_at: DateTime<Utc>,
    pub deletion_at: DateTime<Utc>,
}
