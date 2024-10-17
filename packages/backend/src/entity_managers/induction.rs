use chrono::{Duration, Utc};
use palform_entities::{
    admin_public_key, form, organisation, organisation_invite, prelude::*, team,
};
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait,
};

pub struct InductionStatusManager<'a, T: ConnectionTrait> {
    user_id: PalformDatabaseID<IDAdminUser>,
    org_id: PalformDatabaseID<IDOrganisation>,
    conn: &'a T,
}

impl<'a, T: ConnectionTrait> InductionStatusManager<'a, T> {
    pub fn new(
        user_id: PalformDatabaseID<IDAdminUser>,
        org_id: PalformDatabaseID<IDOrganisation>,
        conn: &'a T,
    ) -> Self {
        Self {
            user_id,
            org_id,
            conn,
        }
    }

    pub async fn has_created_key(&self) -> Result<bool, DbErr> {
        AdminPublicKey::find()
            .filter(
                Condition::all()
                    .add(admin_public_key::Column::OrganisationId.eq(self.org_id))
                    .add(admin_public_key::Column::UserId.eq(self.user_id)),
            )
            .count(self.conn)
            .await
            .map(|c| c > 0)
    }

    pub async fn can_create_invite(&self) -> Result<bool, DbErr> {
        let membership =
            OrganisationMembership::find_by_id((self.org_id, self.user_id))
                .one(self.conn)
                .await?
                .ok_or(DbErr::RecordNotFound(
                    "User is not member of org".to_string(),
                ))?;

        Ok(membership.is_admin)
    }

    pub async fn has_created_invite(&self) -> Result<bool, DbErr> {
        OrganisationInvite::find()
            .filter(organisation_invite::Column::OrganisationId.eq(self.org_id))
            .count(self.conn)
            .await
            .map(|c| c > 0)
    }

    pub async fn has_created_form(&self) -> Result<bool, DbErr> {
        Form::find()
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(
                Condition::all()
                    .add(team::Column::IsDefault.eq(true))
                    .add(team::Column::OrganisationId.eq(self.org_id)),
            )
            .count(self.conn)
            .await
            .map(|c| c > 0)
    }

    pub async fn induction_period_expired(&self) -> Result<bool, DbErr> {
        Organisation::find_by_id(self.org_id)
            .filter(organisation::Column::CreatedAt.gt(Utc::now() - Duration::days(7)))
            .count(self.conn)
            .await
            .map(|c| c == 0)
    }

    pub async fn has_active_key(&self) -> Result<bool, DbErr> {
        AdminPublicKey::find()
            .filter(
                Condition::all()
                    .add(admin_public_key::Column::OrganisationId.eq(self.org_id))
                    .add(admin_public_key::Column::UserId.eq(self.user_id))
                    .add(admin_public_key::Column::ExpiresAt.gt(Utc::now())),
            )
            .count(self.conn)
            .await
            .map(|c| c > 0)
    }
}
