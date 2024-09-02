use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TeamAsset::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TeamAsset::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TeamAsset::TeamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_asset_team")
                            .from(TeamAsset::Table, TeamAsset::TeamId)
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(TeamAsset::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TeamAsset::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum TeamAsset {
    Table,
    Id,
    TeamId,
    CreatedAt,
}
