use chrono::{NaiveDateTime, Utc};
use palform_entities::{organisation, organisation_invite, prelude::*};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationInvite},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::organisation_invite::{
    APIOrganisationInvite, APIOrganisationInvitePreview,
};

pub struct OrganisationInviteManager;

impl OrganisationInviteManager {
    pub async fn list<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIOrganisationInvite>, DbErr> {
        OrganisationInvite::find()
            .filter(organisation_invite::Column::OrganisationId.eq(org_id))
            .order_by(organisation_invite::Column::CreatedAt, Order::Desc)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        expires_at: NaiveDateTime,
        single_use: bool,
    ) -> Result<organisation_invite::Model, DbErr> {
        let new_invite = organisation_invite::ActiveModel {
            id: Set(PalformDatabaseID::<IDOrganisationInvite>::random()),
            organisation_id: Set(org_id),
            single_use: Set(single_use),
            expires_at: Set(expires_at),
            ..Default::default()
        };
        new_invite.insert(conn).await
    }

    pub async fn verify_invite_org<T: ConnectionTrait>(
        conn: &T,
        invite_id: PalformDatabaseID<IDOrganisationInvite>,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<bool, DbErr> {
        OrganisationInvite::find_by_id(invite_id)
            .filter(organisation_invite::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDOrganisationInvite>,
    ) -> Result<(), DbErr> {
        OrganisationInvite::delete_by_id(id)
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDOrganisationInvite>,
    ) -> Result<Option<organisation_invite::Model>, DbErr> {
        OrganisationInvite::find_by_id(id).one(conn).await
    }

    pub async fn preview<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDOrganisationInvite>,
    ) -> Result<Option<APIOrganisationInvitePreview>, DbErr> {
        OrganisationInvite::find_by_id(id)
            .filter(organisation_invite::Column::ExpiresAt.gt(Utc::now().naive_utc()))
            .join(
                JoinType::InnerJoin,
                organisation_invite::Relation::Organisation.def(),
            )
            .column_as(organisation::Column::Id, "org_id")
            .column_as(organisation::Column::DisplayName, "org_display_name")
            .into_model()
            .one(conn)
            .await
    }
}
