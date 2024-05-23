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
#[derive(serde::Serialize, Debug)]
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

    dbg!(&days);
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
    dbg!(&past_days);

    // 4. Get the amount of each ingredient that will have some surplus in the current range.
    let mut surplus = utils::calculate_surplus_on_date(past_days.clone(), from.clone());
    dbg!(&surplus);

    // 5. For each ingredient, check the current surplus is enough.
    // Note - could we reuse calculate surplus somehow?

    let mut purchases_needed: HashMap<i32, IngredientPurchase> = HashMap::new();

    for ingredient in days
        .iter()
        .flat_map(|(_, i)| i)
        .collect::<Vec<&Ingredient>>()
    {
        // Check if we've encountered the ingredient yet.
        if let Some(ingredient_purchase) = purchases_needed.get_mut(&ingredient.ingredient_id) {
            ingredient_purchase.ingredient.quantity = Some(
                ingredient_purchase.ingredient.quantity.unwrap_or(0.0)
                    + ingredient.quantity.unwrap(),
            );
        }
        // If not, set the initial amount to the surplus at the start of the week.
        // A negative value is used so that the surplus is 'used up' before purchases are needed.
        else {
            let initial_amount: Option<f64> = match surplus.get(&ingredient.ingredient_id) {
                Some(ingredient) => Some(-ingredient.quantity.unwrap_or(0.0)),
                None => Some(0.0),
            };
            purchases_needed
        }
    }
    todo!()
}
