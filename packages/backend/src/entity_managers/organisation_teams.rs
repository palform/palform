use palform_entities::{
    admin_user, prelude::*, sea_orm_active_enums::OrganisationMemberRoleEnum, team, team_membership,
};
use palform_migration::all;
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::organisation_team::{
    APIOrganisationTeam, APIOrganisationTeamMember, APIOrganisationTeamMembership,
};

use super::billing_entitlement_proxy::BillingEntitlementCountTrait;

impl BillingEntitlementCountTrait for OrganisationTeamsManager {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<u64, DbErr> {
        Team::find()
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
    }
}

pub struct OrganisationTeamsManager;

impl OrganisationTeamsManager {
    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<Option<APIOrganisationTeam>, DbErr> {
        Team::find_by_id(team_id)
            .join(JoinType::LeftJoin, team::Relation::TeamMembership.def())
            .column_as(team_membership::Column::UserId.count(), "num_members")
            .group_by(team::Column::Id)
            .into_model()
            .one(conn)
            .await
    }

    pub async fn list_member_teams<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Vec<APIOrganisationTeamMembership>, DbErr> {
        let resp = Team::find()
            .find_also_related(team_membership::Entity)
            .filter(
                Condition::all()
                    .add(team_membership::Column::UserId.eq(user_id))
                    .add(team::Column::OrganisationId.eq(org_id)),
            )
            .all(conn)
            .await?;

        let vec: Result<Vec<APIOrganisationTeamMembership>, DbErr> = resp
            .iter()
            .map(|(t, tm)| -> Result<APIOrganisationTeamMembership, DbErr> {
                let tm = tm.clone().ok_or(DbErr::RecordNotFound(
                    "No matching membership for team".to_string(),
                ))?;
                Ok(APIOrganisationTeamMembership {
                    team_id: t.id.clone(),
                    name: t.name.clone(),
                    my_role: tm.role,
                })
            })
            .collect();

        Ok(vec?)
    }

    pub async fn list_org_teams<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIOrganisationTeam>, DbErr> {
        Team::find()
            .join(JoinType::LeftJoin, team::Relation::TeamMembership.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .column_as(team_membership::Column::UserId.count(), "num_members")
            .group_by(team::Column::Id)
            .order_by(team::Column::IsDefault, Order::Asc)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get_org_default_team<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<team::Model, DbErr> {
        Team::find()
            .filter(
                Condition::all()
                    .add(team::Column::OrganisationId.eq(org_id))
                    .add(team::Column::IsDefault.eq(true)),
            )
            .one(conn)
            .await
            .map(|v| {
                v.ok_or(DbErr::RecordNotFound(
                    "Organisation must have a default team".to_string(),
                ))
            })?
    }

    pub async fn verify_team_org<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<bool, DbErr> {
        Team::find_by_id(team_id)
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        name: String,
        is_default: bool,
    ) -> Result<team::Model, DbErr> {
        let new_team = team::ActiveModel {
            id: Set(PalformDatabaseID::<IDTeam>::random()),
            name: Set(name),
            organisation_id: Set(org_id),
            is_default: Set(if is_default { Some(is_default) } else { None }),
        };

        new_team.insert(conn).await
    }

    fn active_model_from_params(
        team_id: PalformDatabaseID<IDTeam>,
        user_id: PalformDatabaseID<IDAdminUser>,
        role: OrganisationMemberRoleEnum,
    ) -> team_membership::ActiveModel {
        team_membership::ActiveModel {
            team_id: Set(team_id),
            user_id: Set(user_id),
            role: Set(role),
        }
    }

    pub async fn list_members_for_team<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<Vec<APIOrganisationTeamMember>, DbErr> {
        TeamMembership::find()
            .filter(team_membership::Column::TeamId.eq(team_id))
            .join(
                JoinType::InnerJoin,
                team_membership::Relation::AdminUser.def(),
            )
            .column_as(admin_user::Column::Id, "user_id")
            .column_as(admin_user::Column::DisplayName, "user_display_name")
            .column_as(admin_user::Column::Email, "user_email")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn verify_is_member<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<bool, DbErr> {
        TeamMembership::find_by_id((team_id, user_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn add_member_to_team<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        user_id: PalformDatabaseID<IDAdminUser>,
        role: OrganisationMemberRoleEnum,
    ) -> Result<(), DbErr> {
        let new_membership = Self::active_model_from_params(team_id, user_id, role);
        new_membership.insert(conn).await?;
        Ok(())
    }

    pub async fn change_member_role<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        user_id: PalformDatabaseID<IDAdminUser>,
        role: OrganisationMemberRoleEnum,
    ) -> Result<(), DbErr> {
        let membership = Self::active_model_from_params(team_id, user_id, role);
        membership.update(conn).await?;
        Ok(())
    }

    pub async fn remove_from_team<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        TeamMembership::delete_by_id((team_id, user_id))
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn remove_from_all_teams<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<(), DbErr> {
        let teams: Vec<PalformDatabaseID<IDTeam>> = TeamMembership::find()
            .join(JoinType::InnerJoin, team_membership::Relation::Team.def())
            .filter(all![
                team::Column::OrganisationId.eq(org_id),
                team_membership::Column::UserId.eq(user_id.clone())
            ])
            .select_only()
            .column(team_membership::Column::TeamId)
            .into_tuple()
            .all(conn)
            .await?;

        TeamMembership::delete_many()
            .filter(all![
                team_membership::Column::TeamId.is_in(teams),
                team_membership::Column::UserId.eq(user_id.clone())
            ])
            .exec(conn)
            .await?;
        Ok(())
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<(), DbErr> {
        Team::delete_by_id(team_id).exec(conn).await.map(|_| ())
    }
}
