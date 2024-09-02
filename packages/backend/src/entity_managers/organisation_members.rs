use palform_entities::{admin_user, organisation_membership, prelude::*};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::organisation_member::APIOrgMember;

use super::billing_entitlement_proxy::BillingEntitlementCountTrait;

impl BillingEntitlementCountTrait for OrganisationMembersManager {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<u64, DbErr> {
        OrganisationMembership::find()
            .filter(organisation_membership::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
    }
}

pub struct OrganisationMembersManager;

impl OrganisationMembersManager {
    pub async fn list_all<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIOrgMember>, DbErr> {
        OrganisationMembership::find()
            .filter(organisation_membership::Column::OrganisationId.eq(org_id))
            .join(
                JoinType::LeftJoin,
                organisation_membership::Relation::AdminUser.def(),
            )
            .column_as(admin_user::Column::DisplayName, "user_display_name")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        OrganisationMembership::delete_by_id((org_id, user_id))
            .exec(conn)
            .await
            .map(|_| ())
    }

    fn active_model_from_attr(
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
        is_admin: bool,
    ) -> organisation_membership::ActiveModel {
        organisation_membership::ActiveModel {
            user_id: Set(user_id),
            organisation_id: Set(org_id),
            is_admin: Set(is_admin),
            ..Default::default()
        }
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
        is_admin: bool,
    ) -> Result<(), DbErr> {
        Self::active_model_from_attr(org_id, user_id, is_admin)
            .insert(conn)
            .await
            .map(|_| ())
    }

    pub async fn check_is_member<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<bool, DbErr> {
        OrganisationMembership::find_by_id((org_id, user_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn get_is_admin<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Option<bool>, DbErr> {
        OrganisationMembership::find_by_id((org_id, user_id))
            .select_only()
            .column(organisation_membership::Column::IsAdmin)
            .into_tuple()
            .one(conn)
            .await
    }

    pub async fn set_is_admin<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
        is_admin: bool,
    ) -> Result<(), DbErr> {
        let updated_member = organisation_membership::ActiveModel {
            organisation_id: Set(org_id),
            user_id: Set(user_id),
            is_admin: Set(is_admin),
            ..Default::default()
        };

        updated_member.update(conn).await?;
        Ok(())
    }
}
