use serde::{Deserialize, Serialize};
use sqlx::types;
use std::{collections::HashMap, error::Error};

use super::ingredient::{Ingredient, IngredientQuantity};

/// Represents a recipe with fully populated ingredient data.
#[derive(Debug, Serialize, Clone, Eq)]
pub struct Recipe {
    /// The recipe's unique ID.
    pub id: i32,
    /// The display name of the recipe.
    pub name: String,
    /// The number of portions this recipe makes.
    portions: i32,
    /// A JSON array of strings representing the recipe's steps.
    steps: types::JsonValue,
    /// A list of the recipe's ingredients.
    pub ingredients: Vec<IngredientQuantity>,
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Recipe {
    pub fn new(
        id: i32,
        name: String,
        portions: i32,
        steps: types::JsonValue,
        ingredients: Vec<IngredientQuantity>,
    ) -> Self {
        Recipe {
            id,
            name,
            portions,
            steps,
            ingredients,
        }
    }
}

/// An input to insert a recipe into the database.
#[derive(Deserialize, Debug)]
pub struct RecipeInput {
    /// The display name of the recipe.
    pub name: String,
    /// The number of portions the recipe makes.
    pub portions: i32,
    /// A JSON array of strings representing the recipe's steps.
    pub steps: types::JsonValue,
    /// Represents the ingredients of the recipe as relations on ID.
    pub ingredients: Vec<RecipeIngredientInput>,
}

/// Represents a single recipe/ingredient relation, with quantity.
/// - `id` - The unique ID of the ingredient
/// - `quantity` - The amount of the ingredient used in whatever recipe the relation is linked to
#[derive(Deserialize, Debug)]
pub struct RecipeIngredientInput {
    pub id: i32,
    pub quantity: i32,
}

impl RecipeIngredientInput {
    pub fn get_id(&self) -> i32 {
        self.id
    }
    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}

pub async fn query_multiple(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
) -> Result<Vec<Recipe>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT
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
		FROM recipe as r
			LEFT JOIN recipe_ingredient as ri 
				ON r.id = ri.recipe_id
			LEFT JOIN ingredient as i 
				ON ri.ingredient_id = i.id
		WHERE r.user_id = $1 AND ingredient_id IS NOT NULL
		ORDER BY r.id DESC
		",
        user_id
    )
    .fetch_all(pool)
    .await?;

    // Group rows with the same ID into single recipes with a list of ingredients
    let mut recipe_map = HashMap::<i32, Recipe>::new();

    for row in rows {
        let recipe = Recipe {
            id: row.recipe_id,
            name: row.recipe_name,
            portions: row.recipe_portions,
            steps: row.recipe_steps,
            ingredients: vec![],
        };

        let ingredient = Ingredient::new(
            row.ingredient_id,
            row.ingredient_name,
            row.user_id,
            row.ingredient_unit,
            row.ingredient_minimum_quantity,
            row.ingredient_purchase_quantity,
            row.ingredient_life,
        );

        let ingredient_quantity = IngredientQuantity::new(ingredient, row.ingredient_quantity);

        // Checks the hashmap for the current recipe ID, creating a new entry if one does not exist
        let entry = recipe_map.entry(row.recipe_id).or_insert(recipe);
        entry.ingredients.push(ingredient_quantity);
    }
    let mut recipes: Vec<Recipe> = recipe_map.values().into_iter().cloned().collect();
    recipes.sort_by(|a, b| a.id.cmp(&b.id));
    // Collect the map values - we can safely discard the keys
    Ok(recipes)
}

pub async fn query_single(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    recipe_id: i32,
) -> Result<Recipe, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT
			r.id, 
			r.name,
			r.portions, 
			r.steps,

			i.id as ingredient_id,
			i.name as ingredient_name,
            i.user_id,
			i.unit as ingredient_unit,
			i.minimum_quantity as ingredient_minimum_quantity,
			i.purchase_quantity as ingredient_purchase_quantity,
			i.life as ingredient_life,
			ri.quantity as ingredient_quantity
		FROM recipe as r
			LEFT JOIN recipe_ingredient as ri ON r.id = ri.recipe_id
			LEFT JOIN ingredient as i ON ri.ingredient_id = i.id
		WHERE r.user_id = $1 AND r.id = $2 AND ingredient_id IS NOT NULL
		",
        user_id,
        recipe_id
    )
    .fetch_all(pool)
    .await?;

    // If a row is present, the recipe exists. If not, return None
    match rows.get(0) {
        Some(first_row) => {
            let mut recipe = Recipe {
                id: first_row.id,
                name: first_row.name.clone(),
                portions: first_row.portions,
                steps: first_row.steps.clone(),
                ingredients: vec![],
            };

            for row in rows {
                let ingredient = Ingredient::new(
                    row.ingredient_id,
                    row.ingredient_name,
                    row.user_id,
                    row.ingredient_unit,
                    row.ingredient_minimum_quantity,
                    row.ingredient_purchase_quantity,
                    row.ingredient_life,
                );

                let ingredient_quantity =
                    IngredientQuantity::new(ingredient, row.ingredient_quantity);

                // Checks the hashmap for the current recipe ID, creating a new entry if one does not exist
                recipe.ingredients.push(ingredient_quantity);
            }
            Ok(recipe)
        }
        None => Err(sqlx::Error::RowNotFound),
    }
}

/// Creates a recipe in the database. Returns it's generated ID
pub async fn create(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    recipe: RecipeInput,
) -> Result<i32, sqlx::Error> {
    // Extract vectors of ingredient ID and quantity from the recipe struct
    let ingredient_ids: Vec<i32> = recipe
        .ingredients
        .iter()
        .map(RecipeIngredientInput::get_id)
        .collect();

    let ingredient_quantities: Vec<i32> = recipe
        .ingredients
        .iter()
        .map(RecipeIngredientInput::get_quantity)
        .collect();

    // A slightly complex query:
    // 1. Insert a row into the recipe table, returning its new ID
    // 2. Insert all ingredient and quantity IDs into the recipe_ingredient table,
    //    using the postgres UNNEST function. Much quicker than iterating over ingredients.
    let insertion = sqlx::query!(
        "WITH recipe AS (
			INSERT INTO recipe (user_id, name, portions, steps)
			VALUES ($1, $2, $3, $4)
			RETURNING id
		)
		INSERT INTO recipe_ingredient (recipe_id, ingredient_id, quantity)
			SELECT recipe.id, ingredient.id, ingredient.quantity 
			FROM recipe, UNNEST($5::integer[], $6::integer[]) as ingredient(id, quantity)
		RETURNING recipe_id
	",
        user_id,
        recipe.name,
        recipe.portions,
        recipe.steps,
        &ingredient_ids,
        &ingredient_quantities
    )
    .fetch_one(pool)
    .await?;

    // Return the ID of the created recipe
    Ok(insertion.recipe_id)
}

/// Creates a recipe in the database. Returns it's generated ID
pub async fn update(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    recipe_id: i32,
    recipe: RecipeInput,
) -> Result<(), Box<dyn Error>> {
    // Extract vectors of ingredient ID and quantity from the recipe struct
    let ingredient_ids: Vec<i32> = recipe
        .ingredients
        .iter()
        .map(RecipeIngredientInput::get_id)
        .collect();

    let ingredient_quantities: Vec<i32> = recipe
        .ingredients
        .iter()
        .map(RecipeIngredientInput::get_quantity)
        .collect();

    let mut tx = pool.begin().await?;

    // Update the recipes table
    sqlx::query!(
        "
		UPDATE recipe SET
			name = $2,
			portions = $3,
			steps = $4
		WHERE id = $1 AND user_id = $5
		
	",
        recipe_id,
        recipe.name,
        recipe.portions,
        recipe.steps,
        user_id
    )
    .execute(&mut **(&mut tx))
    .await?;

    // Delete existing ingredients
    sqlx::query!(
        "DELETE FROM recipe_ingredient WHERE recipe_id = $1	",
        recipe_id,
    )
    .execute(&mut **(&mut tx))
    .await?;

    // Insert new ingredient list
    sqlx::query!(
        "INSERT INTO recipe_ingredient (recipe_id, ingredient_id, quantity)
			SELECT $1, ingredient.id, ingredient.quantity 
			FROM UNNEST($2::integer[], $3::integer[]) as ingredient(id, quantity)",
        recipe_id,
        &ingredient_ids,
        &ingredient_quantities
    )
    .execute(&mut **(&mut tx))
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn delete(
    pool: &sqlx::Pool<sqlx::Postgres>,
    user_id: i32,
    recipe_id: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "DELETE FROM recipe WHERE id = $1 AND user_id = $2
	",
        recipe_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
