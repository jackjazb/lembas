use sea_orm_migration::prelude::*;

use crate::m20240518_150035_unit::Unit;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ingredient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Ingredient::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Ingredient::UnitId).integer()) // This is nullable by design.
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-ingredient-unit")
                            .from(Ingredient::Table, Ingredient::UnitId)
                            .to(Unit::Table, Unit::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(ColumnDef::new(Ingredient::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ingredient::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Ingredient {
    Table,
    Id,
    Name,
    UnitId,
}
