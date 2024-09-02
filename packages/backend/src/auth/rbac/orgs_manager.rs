use palform_entities::prelude::*;
use palform_tsid::{resources::{IDAdminUser, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{http::Status, serde::json::Json};
use sea_orm::{ConnectionTrait, DbErr, EntityTrait};

use crate::api::error::{APIError, APIInternalError};

pub struct OrgsRBACManager {
    pub org_id: PalformDatabaseID<IDOrganisation>,
    pub user_id: PalformDatabaseID<IDAdminUser>,
}

#[derive(PartialEq)]
enum UserOrgPermission {
    None = 0,
    Member,
    Admin,
}

impl OrgsRBACManager {
    async fn get_user_org_permission<T: ConnectionTrait>(
        &self,
        conn: &T,
    ) -> Result<UserOrgPermission, DbErr> {
        let membership = OrganisationMembership::find_by_id((self.org_id, self.user_id))
            .one(conn)
            .await?;

        if let Some(membership) = membership {
            if membership.is_admin {
                Ok(UserOrgPermission::Admin)
            } else {
                Ok(UserOrgPermission::Member)
            }
        } else {
            Ok(UserOrgPermission::None)
        }
    }

    async fn require_permission<T: ConnectionTrait>(
        &self,
        conn: &T,
        require_admin: bool,
    ) -> Result<bool, DbErr> {
        let permission = self.get_user_org_permission(conn).await?;

        if permission == UserOrgPermission::None {
            return Ok(false);
        }

        if permission == UserOrgPermission::Member && require_admin {
            return Ok(false);
        }

        Ok(true)
    }

    pub async fn require_in_request<T: ConnectionTrait>(
        &self,
        conn: &T,
        require_admin: bool,
    ) -> Result<(), (Status, Json<APIError>)> {
        let granted = self
            .require_permission(conn, require_admin)
            .await
            .map_err(|e| e.to_internal_error())?;

        if granted {
            Ok(())
        } else {
            Err(APIError::NotAllowed.into())
        }
    }
}
