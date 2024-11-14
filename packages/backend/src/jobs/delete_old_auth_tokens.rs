use sea_orm::{DatabaseConnection, DbErr};

use crate::auth::tokens::TokenManager;

pub async fn job_delete_old_auth_tokens(db: &DatabaseConnection) -> Result<(), DbErr> {
    TokenManager::delete_all_old_tokens(db).await
}
