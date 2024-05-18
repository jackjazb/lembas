use sea_orm_migration::prelude::*;

use crate::{m20240518_145641_user::User, m20240518_150332_ingredient::Ingredient};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Recipe::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Recipe::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Recipe::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipe-user")
                            .from(Recipe::Table, Recipe::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Recipe::Title).string().not_null())
                    .col(ColumnDef::new(Recipe::Portions).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(RecipeIngredient::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RecipeIngredient::RecipeId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipeingredient-recipe")
                            .from(RecipeIngredient::Table, RecipeIngredient::RecipeId)
                            .to(Recipe::Table, Recipe::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredient::IngredientId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-recipeingredient-ingredient")
                            .from(RecipeIngredient::Table, RecipeIngredient::IngredientId)
                            .to(Ingredient::Table, Ingredient::Id)
                            .on_delete(ForeignKeyAction::Restrict),
                    )
                    .col(
                        ColumnDef::new(RecipeIngredient::Quantity)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Recipe::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Recipe {
    Table,
    Id,
    UserId,
    Title,
    Portions,
}

#[derive(DeriveIden)]
enum RecipeIngredient {
    Table,
    RecipeId,
    IngredientId,
    Quantity,
}
