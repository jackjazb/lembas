use std::collections::HashMap;

use super::ingredient::{DayIngredients, Ingredient};

/// All possible user deletable entities
pub enum UserDeletable {
    Recipe,
    Ingredient,
    Day,
    Reminder,
}

impl ToString for UserDeletable {
    fn to_string(&self) -> String {
        match self {
            UserDeletable::Recipe => "recipe",
            UserDeletable::Ingredient => "ingredient",
            UserDeletable::Day => "day",
            UserDeletable::Reminder => "reminder",
        }
        .into()
    }
}

/// Reusable function to delete a given entity by ID.
pub async fn delete_entity(
    pool: &sqlx::PgPool,
    entity: UserDeletable,
    account_id: i32,
    entity_id: i32,
) -> Result<(), sqlx::Error> {
    // Produces `DELETE FROM <table> WHERE id = <entity_id> AND account_id = <account_id>`.
    let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new("DELETE FROM ");
    let mut sep = qb.separated(" ");
    sep.push(entity.to_string());
    sep.push("WHERE id =");
    sep.push_bind(entity_id);
    sep.push("AND account_id =");
    sep.push_bind(account_id);
    let query = qb.build();
    query.execute(pool).await?;
    Ok(())
}

/// Calculates the amount of each ingredient left over on the passed date.
/// Returns a map of ingredient ID to ingredient.
pub fn calculate_surplus_on_date(
    days: Vec<DayIngredients>,
    date: chrono::NaiveDate,
) -> HashMap<i32, Ingredient> {
    let mut surplus: HashMap<i32, Ingredient> = HashMap::new();
    for (day, ingredients) in days {
        let days_since = (date - day).num_days() as i32;
        for ingredient in ingredients {
            // Check if the ingredient is purchasable and in date.
            if ingredient.purchase_unit == 0.0 || ingredient.life < days_since {
                continue;
            }

            // Retrieve the current ingredient from the hashmap.
            // If this is the first instance, insert a clone with zero quantity.
            let current = surplus
                .entry(ingredient.ingredient_id)
                .or_insert(ingredient.clone().zeroed_clone());

            // Subtract the amount used on the current date from the current quantity.
            current.quantity -= ingredient.quantity;

            // If the current quantity falls below 0, add purchase units until it's in the green.
            while current.quantity < 0.0 {
                current.quantity += ingredient.purchase_unit;
            }
        }
    }

    // Return the completed hash map.
    surplus
}
