use std::{collections::HashMap, error::Error};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::{error, info};

use crate::service::{ingredient::Ingredient, utils};

use super::{day::DayRangeParams, router::ServerError};

/// Represents an ingredient that needs to be purchased.
/// `units` represents the number of purchase units that are required to fulfil the range's requirements.
/// The ingredient's quantity field refers to the amount of ingredient used.
/// For example, for Flour, which has a purhcase unit of 1500.0:
/// 	- quantity = 500.0
/// 	- purchase_unit = 1 (i.e. 1 * 1500.0)
#[derive(serde::Serialize, Debug, Clone)]
pub struct IngredientPurchase {
    ingredient: Ingredient,
    units: i32,
}

/// Handler for retrieving scheduled recipes in a date range.
pub async fn get_list_for_range(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Query(range): Query<DayRangeParams>,
) -> Result<Json<Vec<IngredientPurchase>>, ServerError> {
    let result = build_list(&pool, account_id, range.from, range.to).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

async fn build_list(
    pool: &sqlx::PgPool,
    account_id: i32,
    from: chrono::NaiveDate,
    to: chrono::NaiveDate,
) -> Result<Vec<IngredientPurchase>, Box<dyn Error>> {
    // 1. Query the day table to obtain the ingredients used on each day.
    let days = Ingredient::find_used_in_range(pool, account_id, from.clone(), to.clone()).await?;

    // 2. Identify the longest lived ingredient used.
    let longest_shelf_life: i32 = days
        .iter()
        .flat_map(|(_, i)| i)
        .map(|i| i.life)
        .max()
        .ok_or("failed to identify longest shelf life")?;

    info!("querying {longest_shelf_life} days");

    // 3. Query again for days containing every past use of the current range's ingredients that are still in date.
    let past_days = Ingredient::find_used_in_range(
        pool,
        account_id,
        from - chrono::Duration::days(longest_shelf_life.into()),
        from.clone() - chrono::Duration::days(1),
    )
    .await?;

    // 4. Get the amount of each ingredient that will have some surplus in the current range.
    let surplus = utils::calculate_surplus_on_date(past_days.clone(), from.clone());

    // 5. For each ingredient, check the current surplus is enough, otherwise add purchases.

    let mut purchases_needed: HashMap<i32, IngredientPurchase> = HashMap::new();

    for ingredient in days
        .iter()
        .flat_map(|(_, i)| i)
        .collect::<Vec<&Ingredient>>()
    {
        // If this is the first time we've encountered the ingredient, add a new entry.
        if !purchases_needed.contains_key(&ingredient.ingredient_id) {
            let mut ingredient_entry = ingredient.clone().zeroed_clone();

            // Subtract any existing surplus from the initial amount needed, otherwise start at 0.
            ingredient_entry.quantity = match surplus.get(&ingredient_entry.ingredient_id) {
                Some(i) => -i.quantity,
                None => 0.0,
            };

            let purchase = IngredientPurchase {
                ingredient: ingredient_entry,
                units: 0,
            };
            purchases_needed.insert(ingredient.ingredient_id, purchase);
        }

        let current_purchase = purchases_needed.get_mut(&ingredient.ingredient_id).unwrap();
        current_purchase.ingredient.quantity += ingredient.quantity;

        // Calculate the number of purchase_units required: quantity/purhcase_unit
        current_purchase.units = current_purchase.ingredient.calculate_units_required();
    }
    Ok(purchases_needed.values().cloned().collect())
}
