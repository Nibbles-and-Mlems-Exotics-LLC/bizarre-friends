mod m20250310_141559_create_species_table;
mod m20250310_152445_create_subspecies_table;
mod m20250310_153102_create_feeding_schedule_table;
mod m20250310_153152_create_food_table;
mod m20250311_131009_create_unit_tables;

pub use sea_orm_migration::prelude::*;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250310_141559_create_species_table::Migration),
            Box::new(m20250310_152445_create_subspecies_table::Migration),
            Box::new(m20250310_153102_create_feeding_schedule_table::Migration),
            Box::new(m20250310_153152_create_food_table::Migration),
            Box::new(m20250311_131009_create_unit_tables::Migration),
        ]
    }
}
