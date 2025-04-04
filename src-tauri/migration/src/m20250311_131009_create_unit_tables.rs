use sea_orm_migration::{prelude::*, schema::*, sea_orm::EnumIter};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

enum Unit {
    Table,
    Abbreviation,
    MeasureOf,
    Name,
}

pub enum UnitConversion {
    Table,
    UnitA,
    UnitAOffset,
    UnitABRatio,
    UnitBOffset,
    UnitB,
}

#[derive(Debug, Iden, EnumIter)]
pub enum MeasureOf {
    Mass,
    Length,
    Liquid,
    Volume,
    Temperature,
    Time,
}
