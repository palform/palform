use palform_migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::config::Config;

pub async fn init_db(config: &Config) -> DatabaseConnection {
    let manager = Database::connect(&config.database_url)
        .await
        .expect("Connecting to DB");
    manager.ping().await.expect("Database ping failed");
    Migrator::up(&manager, None)
        .await
        .expect("Performing migrations");
    manager
}
