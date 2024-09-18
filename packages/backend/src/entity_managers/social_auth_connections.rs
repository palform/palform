use palform_entities::prelude::*;
use palform_entities::social_auth_connection;
use palform_tsid::{resources::IDAdminUser, tsid::PalformDatabaseID};
use sea_orm::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter};

use crate::auth::social::SocialAuthService;

pub struct SocialAuthConnectionsManager;

impl SocialAuthConnectionsManager {
    pub async fn list_for_user<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
    ) -> Result<Vec<social_auth_connection::Model>, DbErr> {
        SocialAuthConnection::find()
            .filter(social_auth_connection::Column::UserId.eq(user_id))
            .all(conn)
            .await
    }

    pub async fn add_connection<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        service: SocialAuthService,
        sub: String,
    ) -> Result<(), DbErr> {
        let new_connection = social_auth_connection::ActiveModel {
            user_id: Set(user_id),
            service: Set(service.to_string()),
            sub: Set(sub),
            ..Default::default()
        };
        new_connection.insert(conn).await?;
        Ok(())
    }
}
