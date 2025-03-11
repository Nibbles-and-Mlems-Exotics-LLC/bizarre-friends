use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{EnumIter, Iterable, prelude::Uuid},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Species::Table)
                    .if_not_exists()
                    .col(
                        uuid(Species::Id)
                            .uuid()
                            .unique_key()
                            .not_null()
                            .default(Uuid::new_v4()),
                    )
                    .col(string(Species::Name).primary_key().not_null())
                    .col(
                        string(Species::BinomialName)
                            .not_null()
                            .unique_key()
                            .not_null(),
                    )
                    .col(boolean(Species::IsExotic).default(false).not_null())
                    .col(enumeration(
                        Species::ReproductionType,
                        Alias::new("reproduction_type"),
                        ReproductionType::iter(),
                    ))
                    .col(enumeration(
                        Species::ReproductionMode,
                        Alias::new("reproduction_mode"),
                        ReproductionMode::iter(),
                    ))
                    .col(enumeration(
                        Species::Hermaphroditism,
                        Alias::new("hermaphriditism"),
                        Hermaphroditism::iter(),
                    ))
                    .col(boolean(Species::IsAutogamous).default(false).not_null())
                    .col(boolean(Species::IsSexuallyDimorphic).default(true))
                    .col(unsigned(Species::AverageLifespan).not_null())
                    .col(enumeration(
                        Species::DietType,
                        Alias::new("diet_type"),
                        DietType::iter(),
                    ))
                    .col(string(Species::Appearance))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Species::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Species {
    Table,
    Id,
    Name,
    BinomialName,
    IsExotic,
    ReproductionType,
    ReproductionMode,
    Hermaphroditism,
    IsAutogamous,
    IsSexuallyDimorphic,
    AverageLifespan,
    DietType,
    Appearance,
}

#[derive(Iden, EnumIter)]
pub enum ReproductionType {
    Asexual,
    Sexual,
    Omnisexual,
    Other,
}

#[derive(Iden, EnumIter)]
pub enum ReproductionMode {
    Ovuliparious,           // Eggs, fertalized and developed externally
    Oviparous,              // Eggs, fertalized internally, developed externally
    Ovoviviparous,          // Eggs, fertalized internally, developed internally
    HistotrophicViviparous, // Embryonic, internal, parasitic
    Placental,              // Embryonic, placental
    Marsupial,              // Embryonic, pouched
    Other,
}

#[derive(Iden, EnumIter)]
pub enum Hermaphroditism {
    None,
    Serial,
    Simultaneous,
}

#[derive(Iden, EnumIter)]
pub enum DietType {
    ObligateCarnivore,
    PrimaryCarnivore,
    Omnivore,
    PrimaryHerbivore,
    ObligateHerbivore,
}
