pub use sea_orm_migration::prelude::*;

mod m20240518_145641_user;
mod m20240518_150035_unit;
mod m20240518_150332_ingredient;
mod m20240518_150909_recipe;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240518_145641_user::Migration),
            Box::new(m20240518_150035_unit::Migration),
            Box::new(m20240518_150332_ingredient::Migration),
            Box::new(m20240518_150909_recipe::Migration),
        ]
    }
}
