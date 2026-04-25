use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use chrono::{Duration, Utc};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api::error::{APIError, APIInternalError},
    api_entities::{
        billing::entitlement::APIEntitlementRequest, organisation_invite::APIOrganisationInvite,
    },
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        organisation_invites::OrganisationInviteManager,
    },
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct NewOrganisationInviteRequest {
    pub expires_in_seconds: u32,
    pub single_use: bool,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationInvitesCreatePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Invites",
    operation_id = "organisation.invites.create"
)]
pub async fn organisation_invites_create(
    path: Path<OrganisationInvitesCreatePath>,
    data: Json<NewOrganisationInviteRequest>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    m: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<APIOrganisationInvite>, APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    m.check_entitlement(&txn, APIEntitlementRequest::UserCount)
        .await?;

    OrganisationInviteManager::create(
        db.as_ref(),
        path.org_id,
        Utc::now() + Duration::seconds(i64::from(data.expires_in_seconds)),
        data.single_use,
    )
    .await
    .map(|e| Json(e.into()))
    .map_err(|e| e.to_internal_error())
}
