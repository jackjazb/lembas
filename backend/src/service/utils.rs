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
            // Check if the ingredient is in date.
            if ingredient.life < days_since {
                continue;
            };

            // Retrieve the current ingredient from the hashmap, or insert if this is the first instance.
            if !surplus.contains_key(&ingredient.ingredient_id) {
                let mut new_entry = ingredient.clone();
                new_entry.quantity = Some(0.0);
                surplus.insert(ingredient.ingredient_id, new_entry);
            }
            let current = surplus.get_mut(&ingredient.ingredient_id).unwrap();

            // Get the amount of the ingredient used on the current date.
            let used_quantity = ingredient.quantity.unwrap_or(0.0);

            // Subtract it from the current quantity. If the current quantity falls below 0, add 1 purchase unit.
            match current.quantity {
                Some(quantity) => {
                    // Skip these (e.g. Water).
                    if ingredient.purchase_unit == 0.0 {
                        break;
                    }
                    let mut new_quantity = quantity - used_quantity;

                    while new_quantity < 0.0 {
                        new_quantity += ingredient.purchase_unit;
                        println!("new quantity = {new_quantity}");
                    }
                    current.quantity = Some(new_quantity);
                }
                // This shouldn't happen, but if it does we can just take the current use as the first.
                None => current.quantity = Some(used_quantity),
            }
        }
    }
    surplus
}
