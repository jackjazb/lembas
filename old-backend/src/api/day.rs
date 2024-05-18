use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, time::Duration};

use super::{
    ingredient::{self, Ingredient, IngredientQuantity},
    recipe::Recipe,
    utils::{day_diff, parse_date, sub_days},
};

#[derive(Serialize, Clone, Debug)]
pub struct Day {
    pub date: String,
    pub recipes: Vec<Recipe>,
}

impl Day {
    pub fn new(date: String, recipes: Vec<Recipe>) -> Self {
        Day { date, recipes }
    }
}

/// Contains mapped ingredient lists for quick modification.
pub struct MappedShoppingList {
    /// Tracks surplus of ingredients from previous weeks. Reduced as recipes are added.
    surplus: HashMap<i32, IngredientQuantity>,
    /// Maps ingredient IDs to purchase quantities
    ingredients: HashMap<i32, IngredientPurchaseQuantity>,
    /// Stores additional scheduled ingredients
    scheduled_ingredients: HashMap<i32, IngredientPurchaseQuantity>,
}

impl MappedShoppingList {
    /// Takes a map of initially available ingredients, to which recipes can be added.
    pub fn new(surplus: HashMap<i32, IngredientQuantity>) -> Self {
        MappedShoppingList {
            surplus,
            ingredients: HashMap::new(),
            scheduled_ingredients: HashMap::new(),
        }
    }

    /// Adds a recipe's ingredients to the existing list. An existing surplus is also passed in,
    /// with ingredients only being purchased if the surplus does not cover the amount
    /// required by the recipe.
    pub fn add_recipe(&mut self, recipe: Recipe) {
        for ingredient_quantity in recipe.ingredients {
            let existing_surplus = self.surplus.get_mut(&ingredient_quantity.ingredient.id);

            let entry = self
                .ingredients
                .entry(ingredient_quantity.ingredient.id)
                // If no entry exists, insert a new one with the existing surplus set to the ingredient's initial surplus value.
                .or_insert(match existing_surplus {
                    Some(surplus) => {
                        let mut entry =
                            IngredientPurchaseQuantity::new(ingredient_quantity.ingredient.clone());
                        entry.set_existing_surplus(surplus.quantity);
                        entry
                    }
                    None => IngredientPurchaseQuantity::new(ingredient_quantity.ingredient.clone()),
                });

            // Calculate if we need to purchase more by subtracting the amount of the ingredient
            // we have left from the amount required for this recipe
            let needed_quantity = match self.surplus.get_mut(&ingredient_quantity.ingredient.id) {
                Some(existing_surplus) => {
                    existing_surplus.quantity -= ingredient_quantity.quantity;
                    let mut purchase = 0;
                    if existing_surplus.quantity < 0 {
                        purchase = existing_surplus.quantity.abs();
                        existing_surplus.quantity = 0;
                    }
                    purchase
                }
                None => ingredient_quantity.quantity,
            };

            // Increase the amount to be purchased
            entry.add_puchase_quantity(needed_quantity);

            // Increase the exact amount used
            entry.add_quantity(ingredient_quantity.quantity);
        }
    }

    /// Add a single purchase quantity of the given ingredient to the scheduled ingredient list.
    pub fn add_scheduled_ingredient(&mut self, ingredient: Ingredient) {
        let entry = self
            .scheduled_ingredients
            .entry(ingredient.id)
            .or_insert(IngredientPurchaseQuantity::new(ingredient.clone()));
        entry.add_puchase_quantity(ingredient.purchase_quantity);
        // Increase the exact amount used
        entry.add_quantity(ingredient.purchase_quantity);
    }
}

/// A more usable version of `ShoppingList` that is returned by the API.
#[derive(Serialize, Debug)]
pub struct ShoppingList {
    pub ingredients: Vec<IngredientPurchaseQuantity>,
    pub scheduled_ingredients: Vec<IngredientPurchaseQuantity>,
}

impl Into<ShoppingList> for MappedShoppingList {
    fn into(self) -> ShoppingList {
        ShoppingList {
            ingredients: self.ingredients.values().into_iter().cloned().collect(),
            scheduled_ingredients: self
                .scheduled_ingredients
                .values()
                .into_iter()
                .cloned()
                .collect(),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct IngredientPurchaseQuantity {
    ingredient: Ingredient,
    pub existing_surplus: i32,
    pub used_quantity: i32,
    pub purchase_quantity: i32,
}

impl IngredientPurchaseQuantity {
    // this should be into() impl
    /// Initialises an IngredientPurchaseQuantity wih 0 used and purchase quantities.
    pub fn new(ingredient: Ingredient) -> Self {
        IngredientPurchaseQuantity {
            ingredient: ingredient,
            existing_surplus: 0,
            used_quantity: 0,
            purchase_quantity: 0,
        }
    }

    /// Increase the amount of ingredient to be purchased. This scales w.r.t the minimum purchase
    /// quantity.
    pub fn add_puchase_quantity(&mut self, purchase_quantity: i32) {
        let scaled_purchase_quantity = self.ingredient.scale_purchase_quantity(purchase_quantity);
        self.purchase_quantity += scaled_purchase_quantity;
    }

    /// Increase the quantity of the ingredient to be used by an exact amount
    pub fn add_quantity(&mut self, used_quantity: i32) {
        self.used_quantity += used_quantity;
    }

    pub fn set_existing_surplus(&mut self, existing_surplus: i32) {
        self.existing_surplus = existing_surplus;
    }
}

/// Defines the input for adding a recipe to a day.
/// - `recipe_id` - the ID of the recipe to be added
/// - `date` - a string representing the date the recipe should be added to (YYYY-MM-DD)
#[derive(Deserialize)]
pub struct DayInput {
    pub recipe_id: i32,
    pub date: String,
}

/// Queries any planned meals between a range of dates.
pub async fn query_range(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    from: &String,
    to: &String,
) -> Result<Vec<Day>, Box<dyn Error>> {
    let rows = sqlx::query!(
        "SELECT
			day.date,
			r.id as recipe_id, 
			r.name as recipe_name,
			r.portions as recipe_portions, 
			r.steps as recipe_steps,
            
			i.id as ingredient_id,
            i.user_id,
			i.name as ingredient_name,
			i.unit as ingredient_unit,
			i.minimum_quantity as ingredient_minimum_quantity,
			i.purchase_quantity as ingredient_purchase_quantity,
			i.life as ingredient_life,
			ri.quantity as ingredient_quantity
		FROM day 
			LEFT JOIN recipe as r 
				ON day.recipe_id = r.id
			LEFT JOIN recipe_ingredient as ri 
				ON r.id = ri.recipe_id
			LEFT JOIN ingredient as i 
				ON ri.ingredient_id = i.id
		WHERE r.user_id = $1
        AND day.date BETWEEN $2 AND $3
		ORDER BY day.date ASC
		",
        user_id,
        parse_date(&from)?,
        parse_date(&to)?
    )
    .fetch_all(pool)
    .await?;
    // Build a map of all recipes found by the query
    let mut recipe_map = HashMap::<i32, Recipe>::new();

    for row in rows.iter() {
        let ingredient = Ingredient::new(
            row.ingredient_id,
            row.ingredient_name.clone(),
            row.user_id,
            row.ingredient_unit.clone(),
            row.ingredient_minimum_quantity,
            row.ingredient_purchase_quantity,
            row.ingredient_life,
        );

        let ingredient_quantity = IngredientQuantity::new(ingredient, row.ingredient_quantity);

        // Checks the hashmap for the current recipe ID, creating a new entry if one does not exist
        let entry = recipe_map.entry(row.recipe_id).or_insert(Recipe::new(
            row.recipe_id,
            row.recipe_name.clone(),
            row.recipe_portions,
            row.recipe_steps.clone(),
            vec![],
        ));

        if !entry.ingredients.contains(&ingredient_quantity) {
            entry.ingredients.push(ingredient_quantity);
        }
    }

    // Group the list of day/recipe combinations into a map of dates to lists of recipes.
    let mut day_map = HashMap::<String, Day>::new();

    for row in rows {
        // Fetch the recipe from the map
        if let Some(recipe) = recipe_map.get(&row.recipe_id) {
            let entry = day_map
                .entry(row.date.to_string())
                .or_insert(Day::new(row.date.to_string(), vec![]));
            if !entry.recipes.contains(recipe) {
                entry.recipes.push(recipe.clone());
            }
        }
    }

    let mut days: Vec<Day> = day_map.values().into_iter().cloned().collect();
    days.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(days)
}

// Creates a new day/recipe entry. Also verifies the passed user_id owns the recipe.
pub async fn create(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    day: DayInput,
) -> Result<(), Box<dyn Error>> {
    let date = parse_date(&day.date)?;

    sqlx::query!(
        "INSERT INTO day (recipe_id, date) SELECT $1, $2
		WHERE EXISTS (
			SELECT id FROM recipe WHERE user_id = $3 AND id = $1
		)",
        day.recipe_id,
        date,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Deletes a single recipe from a day.
pub async fn delete_day_recipe(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    date: String,
    recipe_id: i32,
) -> Result<(), Box<dyn Error>> {
    let date = parse_date(&date)?;
    sqlx::query!(
        "DELETE FROM day 
		WHERE recipe_id = $1 
			AND date = $2 
			AND EXISTS (
				SELECT id FROM recipe WHERE user_id = $3 AND id = $1
			)",
        recipe_id,
        date,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Generates a shopping list from a range of days. Surplus from previous weeks
/// is totalled and subtracted from each ingredient's purchase amount.
/// Expired ingredients will not be counted.
pub async fn build_list_for_range(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    from: String,
    to: String,
) -> Result<ShoppingList, Box<dyn Error>> {
    // Days in the current range
    let days = query_range(&pool, user_id, &from, &to).await?;

    // Get the longest possible ingredient lifetime
    let ingredients: Vec<IngredientQuantity> = days
        .iter()
        .flat_map(|day| day.recipes.clone())
        .flat_map(|r| r.ingredients.clone())
        .collect();

    // Get the age of the oldest used ingredient - defaults to 0.
    let days_to_check = ingredients
        .iter()
        .max_by(|a, b| a.ingredient.life.cmp(&b.ingredient.life))
        .map_or(0, |i| i.ingredient.life);

    let surplus_check_start = sub_days(&from, days_to_check)?;
    let surplus_check_end = sub_days(&from, 1)?;

    // Query enough days to catch all potential non-expired surplus
    let surplus_check_days =
        query_range(&pool, user_id, &surplus_check_start, &surplus_check_end).await?;

    // Get the currently available surplus
    let surplus = get_surplus(surplus_check_days, &from)?;

    let mut shopping_list = MappedShoppingList::new(surplus);
    for day in days {
        for recipe in day.recipes {
            shopping_list.add_recipe(recipe);
        }
    }

    // Query the user's ingredient purchasing schedule
    let schedule = ingredient::query_scheduled(&pool, user_id).await?;
    // Check each day in the range for scheduled ingredient occurences and add them.
    let mut day = parse_date(&from)?;
    let target = parse_date(&to);
    while day <= target? {
        for item in schedule.clone().into_iter() {
            let age = day_diff(&day.to_string(), &item.start_date)?;
            if age % item.interval == 0 {
                shopping_list.add_scheduled_ingredient(item.ingredient);
            }
        }
        day += Duration::from_secs(60 * 60 * 24);
    }

    // Sort and return the list, after conversion into the response type
    let mut list: ShoppingList = shopping_list.into();
    list.ingredients
        .sort_by(|a, b| a.ingredient.id.cmp(&b.ingredient.id));
    Ok(list)
}

/// Takes a set of days and an end date, and returns the set of ingredients left over from those days
/// that are still in date.
pub fn get_surplus(
    days: Vec<Day>,
    end_date: &String,
) -> Result<HashMap<i32, IngredientQuantity>, Box<dyn Error>> {
    // Maps ingredient IDs to the quantity available in surplus
    let mut surplus: HashMap<i32, IngredientQuantity> = HashMap::new();

    // Check each ingredient of each day
    for day in days {
        for ingredient in day.recipes.iter().flat_map(|r| r.ingredients.clone()) {
            // Skip the ingredient if it's likely to be out of date
            if ingredient.ingredient.life <= day_diff(&end_date, &day.date)? {
                continue;
            }

            let entry = surplus
                .entry(ingredient.ingredient.id)
                .or_insert(IngredientQuantity::new(ingredient.ingredient.clone(), 0));

            // Reduce the surplus by the amount used
            entry.quantity -= ingredient.quantity;

            // If the quantity is less than 0, assume more of the ingredient was purchased.
            if entry.quantity < 0 {
                // The minumum purchase required to cover the deficit
                let amount_purchased = ingredient
                    .ingredient
                    .scale_purchase_quantity(entry.quantity.abs());

                entry.quantity += amount_purchased;
            }
        }
    }

    Ok(surplus)
}
