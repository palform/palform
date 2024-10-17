use palform_entities::{organisation, organisation_auth_config, prelude::*};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationAuthConfig},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::{
    api_entities::organisation_auth_config::APIOrganisationAuthConfig,
    rocket_util::from_org_id::FromOrgIdTrait,
};

pub struct OrganisationAuthConfigManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for OrganisationAuthConfigManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }
}

impl OrganisationAuthConfigManager {
    pub async fn get<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Option<APIOrganisationAuthConfig>, DbErr> {
        OrganisationAuthConfig::find()
            .join_rev(
                JoinType::InnerJoin,
                organisation::Relation::OrganisationAuthConfig.def(),
            )
            .filter(organisation::Column::Id.eq(self.org_id))
            .into_model::<APIOrganisationAuthConfig>()
            .one(conn)
            .await
    }

    pub async fn check_org_uses_oidc<T: ConnectionTrait>(&self, conn: &T) -> Result<bool, DbErr> {
        OrganisationAuthConfig::find()
            .join_rev(
                JoinType::InnerJoin,
                organisation::Relation::OrganisationAuthConfig.def(),
            )
            .filter(organisation::Column::Id.eq(self.org_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    async fn get_config_id<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Option<PalformDatabaseID<IDOrganisationAuthConfig>>, DbErr> {
        OrganisationAuthConfig::find()
            .join_rev(
                JoinType::InnerJoin,
                organisation::Relation::OrganisationAuthConfig.def(),
            )
            .filter(organisation::Column::Id.eq(self.org_id))
            .select_only()
            .column(organisation_auth_config::Column::Id)
            .into_tuple()
            .one(conn)
            .await
    }

    pub async fn set<T: ConnectionTrait>(
        &self,
        conn: &T,
        data: APIOrganisationAuthConfig,
    ) -> Result<(), DbErr> {
        let existing_id = self.get_config_id(conn).await?;

        let mut model = data.to_active_model();
        if let Some(existing_id) = existing_id {
            model.id = Set(existing_id);
            model.update(conn).await.map(|_| ())
        } else {
            let new_id = PalformDatabaseID::<IDOrganisationAuthConfig>::random();
            model.id = Set(new_id);
            model.insert(conn).await?;

            let updated_org = organisation::ActiveModel {
                id: Set(self.org_id),
                auth_config: Set(Some(new_id)),
                ..Default::default()
            };
            updated_org.update(conn).await.map(|_| ())
        }
    }

    pub async fn delete<T: ConnectionTrait>(&self, conn: &T) -> Result<(), DbErr> {
        let config_id = self.get_config_id(conn).await?;
        if let Some(config_id) = config_id {
            OrganisationAuthConfig::delete_by_id(config_id)
                .exec(conn)
                .await?;
        }

        Ok(())
    }
}
