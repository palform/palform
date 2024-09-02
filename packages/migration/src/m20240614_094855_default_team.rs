use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Team::Table)
                    .add_column(ColumnDef::new(Team::IsDefault).boolean().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("org_team_default_idx")
                    .table(Team::Table)
                    .unique()
                    .col((Team::OrganisationId, IndexOrder::Asc))
                    .col((Team::IsDefault, IndexOrder::Asc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(Index::drop().name("org_team_default_idx").to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Team::Table)
                    .drop_column(Team::IsDefault)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Team {
    Table,
    IsDefault,
    OrganisationId,
}
