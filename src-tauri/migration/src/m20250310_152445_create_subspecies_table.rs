use sea_orm_migration::{prelude::*, schema::*, sea_orm::prelude::Uuid};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Subspecies::Table)
                    .if_not_exists()
                    .col(
                        uuid(Subspecies::Id)
                            .unique_key()
                            .not_null()
                            .default(Uuid::new_v4()),
                    )
                    .col(string(Subspecies::Name).primary_key().not_null())
                    .col(unsigned(Subspecies::AverageLifespan))
                    .col(string(Subspecies::Appearance))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Subspecies::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Subspecies {
    Table,
    Id,
    Name,
    AverageLifespan,
    Appearance,
}
