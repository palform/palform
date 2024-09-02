use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDOrganisationAuthTeamMapping, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationAuthTeamMapping {
    pub id: PalformDatabaseID<IDOrganisationAuthTeamMapping>,
    pub team_id: PalformDatabaseID<IDTeam>,
    pub team_name: String,
    pub role: OrganisationMemberRoleEnum,
    pub field_value: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct APIOrganisationAuthTeamMappingRequest {
    pub team_id: PalformDatabaseID<IDTeam>,
    pub role: OrganisationMemberRoleEnum,
    pub field_value: String,
}
