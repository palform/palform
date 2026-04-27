use palform_entities::{
    audit_log_entry, form, form_branding, form_branding_team_access, organisation,
    organisation_membership, prelude::*, question, question_group,
    sea_orm_active_enums::OrganisationMemberRoleEnum, submission, team, team_asset,
};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType,
    PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
};
use thiserror::Error;

use crate::api_entities::{
    billing::plan::APIBillingSubscription,
    org::{APIOrganisation, APIOrganisationManifest},
};

use super::{
    billing_entitlement_proxy::BillingEntitlementManager,
    organisation_members::OrganisationMembersManager, organisation_teams::OrganisationTeamsManager,
};

#[derive(Error, Debug)]
pub enum BootstrapOrgError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[cfg(feature = "saas")]
    #[error("{0}")]
    BillingError(#[from] crate::billing::error::BillingError),
}

#[derive(Error, Debug)]
pub enum OrgManifestError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[cfg(feature = "saas")]
    #[error("{0}")]
    BillingError(#[from] crate::billing::error::BillingError),
}

pub struct OrganisationManager;
impl OrganisationManager {
    pub async fn list_orgs_for_user<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Vec<APIOrganisation>, DbErr> {
        Organisation::find()
            .join(
                JoinType::InnerJoin,
                organisation::Relation::OrganisationMembership.def(),
            )
            .filter(organisation_membership::Column::UserId.eq(user_id))
            .column_as(organisation::Column::AuthConfig.is_not_null(), "uses_oidc")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get_org_by_id<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Option<APIOrganisation>, DbErr> {
        Organisation::find_by_id(org_id)
            .column_as(organisation::Column::AuthConfig.is_not_null(), "uses_oidc")
            .into_model()
            .one(conn)
            .await
    }

    pub async fn get_org_for_subdomain<T: ConnectionTrait>(
        conn: &T,
        subdomain: String,
    ) -> Result<Option<PalformDatabaseID<IDOrganisation>>, DbErr> {
        Organisation::find()
            .select_only()
            .column(organisation::Column::Id)
            .filter(organisation::Column::Subdomain.eq(subdomain))
            .into_tuple()
            .one(conn)
            .await
    }

    pub async fn rename<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        display_name: String,
    ) -> Result<(), DbErr> {
        let updated_org = organisation::ActiveModel {
            id: Set(org_id),
            display_name: Set(display_name),
            ..Default::default()
        };
        updated_org.update(conn).await?;
        Ok(())
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        display_name: String,
    ) -> Result<PalformDatabaseID<IDOrganisation>, DbErr> {
        let new_org = organisation::ActiveModel {
            id: Set(PalformDatabaseID::<IDOrganisation>::random()),
            display_name: Set(display_name),
            ..Default::default()
        };
        let org = new_org.insert(conn).await?;

        Ok(org.id)
    }

    pub async fn bootstrap_new_org<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        creator_user_id: PalformDatabaseID<IDAdminUser>,

        #[cfg(feature = "saas")] stripe: &stripe::Client,
    ) -> Result<(), BootstrapOrgError> {
        let default_team =
            OrganisationTeamsManager::create(conn, org_id, "Default team".to_string(), true)
                .await?;

        OrganisationMembersManager::create(conn, org_id, creator_user_id, true).await?;

        OrganisationTeamsManager::add_member_to_team(
            conn,
            default_team.id,
            creator_user_id,
            OrganisationMemberRoleEnum::Admin,
        )
        .await?;

        #[cfg(feature = "saas")]
        {
            use crate::actix_util::from_org_id::FromOrgIdTrait;

            let manager = crate::billing::manager::BillingManager::new(stripe);
            manager.register_org_customer_stub(conn, org_id).await?;
            BillingEntitlementManager::new(org_id)
                .create_initial_entitlement(conn)
                .await?;
        }

        Ok(())
    }

    pub async fn set_org_subdomain<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        subdomain: String,
    ) -> Result<(), DbErr> {
        let updated_org = organisation::ActiveModel {
            subdomain: Set(Some(subdomain)),
            ..Default::default()
        };

        let resp = Organisation::update_many()
            .set(updated_org)
            .filter(
                Condition::all()
                    .add(organisation::Column::Id.eq(org_id))
                    .add(organisation::Column::Subdomain.is_null()),
            )
            .exec(conn)
            .await?;

        if resp.rows_affected != 1 {
            return Err(DbErr::RecordNotUpdated);
        }

        Ok(())
    }

    pub async fn get_org_subdomain<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Option<String>, DbErr> {
        Organisation::find_by_id(org_id)
            .select_only()
            .column(organisation::Column::Subdomain)
            .into_tuple()
            .one(conn)
            .await
    }

    pub async fn generate_manifest<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        #[cfg(feature = "saas")] stripe: &stripe::Client,
    ) -> Result<APIOrganisationManifest, OrgManifestError> {
        let form_count = Form::find()
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let submission_count = Submission::find()
            .join(JoinType::InnerJoin, submission::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let question_count = Question::find()
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .join(JoinType::InnerJoin, question_group::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let member_count = OrganisationMembership::find()
            .filter(organisation_membership::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let team_count = Team::find()
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let team_asset_count = TeamAsset::find()
            .join(JoinType::InnerJoin, team_asset::Relation::Team.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let branding_count = FormBranding::find()
            .join(
                JoinType::InnerJoin,
                form_branding::Relation::FormBrandingTeamAccess.def(),
            )
            .join(
                JoinType::InnerJoin,
                form_branding_team_access::Relation::Team.def(),
            )
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        let audit_log_count = AuditLogEntry::find()
            .filter(audit_log_entry::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await?;

        #[allow(unused, reason = "Only unused without feature")]
        let mut active_subscriptions = Vec::<APIBillingSubscription>::default();
        #[cfg(feature = "saas")]
        {
            let manager = crate::billing::manager::BillingManager::new(stripe);
            active_subscriptions = manager.get_org_plans(conn, org_id).await?;
        }

        Ok(APIOrganisationManifest {
            form_count,
            submission_count,
            question_count,
            member_count,
            team_count,
            team_asset_count,
            branding_count,
            audit_log_count,
            active_subscriptions,
        })
    }
}
