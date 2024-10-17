use palform_entities::{organisation_auth_team_mapping, prelude::*, team};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationAuthTeamMapping},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::{
    api_entities::organisation_auth_team_mapping::{
        APIOrganisationAuthTeamMapping, APIOrganisationAuthTeamMappingRequest,
    },
    rocket_util::from_org_id::FromOrgIdTrait,
};

pub struct OrganisationAuthTeamMappingsManager {
    org_id: PalformDatabaseID<IDOrganisation>,
}

impl FromOrgIdTrait for OrganisationAuthTeamMappingsManager {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        Self { org_id }
    }
}

impl OrganisationAuthTeamMappingsManager {
    pub async fn list<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<Vec<APIOrganisationAuthTeamMapping>, DbErr> {
        OrganisationAuthTeamMapping::find()
            .join(
                JoinType::InnerJoin,
                organisation_auth_team_mapping::Relation::Team.def(),
            )
            .filter(team::Column::OrganisationId.eq(self.org_id))
            .column_as(team::Column::Name, "team_name")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn create<T: ConnectionTrait>(
        &self,
        conn: &T,
        request: APIOrganisationAuthTeamMappingRequest,
    ) -> Result<PalformDatabaseID<IDOrganisationAuthTeamMapping>, DbErr> {
        let new_id = PalformDatabaseID::<IDOrganisationAuthTeamMapping>::random();
        let new_mapping = organisation_auth_team_mapping::ActiveModel {
            id: Set(new_id),
            team_id: Set(request.team_id),
            role: Set(request.role),
            field_value: Set(request.field_value),
        };
        new_mapping.insert(conn).await?;
        Ok(new_id)
    }

    pub async fn verify_mapping_org<T: ConnectionTrait>(
        &self,
        conn: &T,
        mapping_id: PalformDatabaseID<IDOrganisationAuthTeamMapping>,
    ) -> Result<bool, DbErr> {
        OrganisationAuthTeamMapping::find_by_id(mapping_id)
            .join(
                JoinType::InnerJoin,
                organisation_auth_team_mapping::Relation::Team.def(),
            )
            .filter(team::Column::OrganisationId.eq(self.org_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn delete<T: ConnectionTrait>(
        &self,
        conn: &T,
        id: PalformDatabaseID<IDOrganisationAuthTeamMapping>,
    ) -> Result<(), DbErr> {
        OrganisationAuthTeamMapping::delete_by_id(id)
            .exec(conn)
            .await
            .map(|_| ())
    }
}
