use log::warn;
use palform_client_common::errors::error::APIErrorWithStatus;
use palform_entities::{prelude::*, sea_orm_active_enums::OrganisationMemberRoleEnum};
use palform_tsid::{resources::{IDAdminUser, IDOrganisation, IDTeam}, tsid::PalformDatabaseID};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, FromQueryResult};
use thiserror::Error;

use crate::{
    api::error::{APIError, APIInternalError},
    auth::tokens::{APIAuthToken, APIAuthTokenSource},
    entity_managers::organisation_teams::OrganisationTeamsManager,
};

#[derive(FromQueryResult)]
struct TeamMembershipResponse {
    role: OrganisationMemberRoleEnum,
}

#[derive(Error, Debug)]
pub enum RBACError {
    #[error("Database: {0}")]
    DBError(#[from] DbErr),
    #[error("Permission refused for user {0} in team {1}")]
    NotAllowed(PalformDatabaseID<IDAdminUser>, PalformDatabaseID<IDTeam>),
}

pub struct TeamsRBACManager {
    user_id: PalformDatabaseID<IDAdminUser>,
}

impl<Source: APIAuthTokenSource> From<APIAuthToken<Source>> for TeamsRBACManager {
    fn from(value: APIAuthToken<Source>) -> Self {
        Self {
            user_id: value.source.get_user_id(),
        }
    }
}

impl TeamsRBACManager {
    async fn get_user_team_permission<T: ConnectionTrait>(
        &self,
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<Option<OrganisationMemberRoleEnum>, DbErr> {
        TeamMembership::find_by_id((team_id, self.user_id))
            .into_model::<TeamMembershipResponse>()
            .one(conn)
            .await
            .map(|r| r.map(|r| r.role))
    }

    async fn require_permission<T: ConnectionTrait>(
        &self,
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        requested_permission: OrganisationMemberRoleEnum,
    ) -> Result<(), RBACError> {
        let actual_permission = Self::get_user_team_permission(self, conn, team_id)
            .await?
            .ok_or(RBACError::NotAllowed(self.user_id, team_id))?;

        let status = match actual_permission {
            OrganisationMemberRoleEnum::Admin => true,
            OrganisationMemberRoleEnum::Editor => {
                requested_permission == OrganisationMemberRoleEnum::Editor
                    || requested_permission == OrganisationMemberRoleEnum::Viewer
            }
            OrganisationMemberRoleEnum::Viewer => {
                requested_permission == OrganisationMemberRoleEnum::Viewer
            }
        };
        if !status {
            return Err(RBACError::NotAllowed(self.user_id, team_id));
        }

        Ok(())
    }

    pub async fn require_in_request<T: ConnectionTrait>(
        &self,
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        org_id: PalformDatabaseID<IDOrganisation>,
        requested_permission: OrganisationMemberRoleEnum,
    ) -> Result<(), APIErrorWithStatus> {
        let team_org_valid = OrganisationTeamsManager::verify_team_org(conn, team_id, org_id)
            .await
            .map_err(|e| e.to_internal_error())?;

        if !team_org_valid {
            return Err(APIError::NotFound.into());
        }

        self.require_permission(conn, team_id, requested_permission)
            .await
            .map_err(|e| {
                match e {
                    RBACError::NotAllowed(_, _) => APIError::NotAllowed,
                    RBACError::DBError(e) => {
                        warn!("rbac: {}", e.to_string());
                        APIError::Internal
                    }
                }
                .into()
            })
    }
}
