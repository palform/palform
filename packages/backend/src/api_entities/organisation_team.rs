use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::resources::{IDAdminUser, IDOrganisation, IDTeam};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationTeam {
    pub id: PalformDatabaseID<IDTeam>,
    pub name: String,
    pub organisation_id: PalformDatabaseID<IDOrganisation>,
    pub num_members: i64,
    pub is_default: Option<bool>,
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationTeamMembership {
    pub team_id: PalformDatabaseID<IDTeam>,
    pub name: String,
    pub my_role: OrganisationMemberRoleEnum,
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIOrganisationTeamMember {
    pub user_id: PalformDatabaseID<IDAdminUser>,
    pub user_display_name: String,
    pub role: OrganisationMemberRoleEnum,
}
