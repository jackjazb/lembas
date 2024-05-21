use super::ingredient::Ingredient;

/// Represents a recipe belonging to a user.
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct Recipe {
    id: i32,
    title: String,
    portions: i32,
    steps: Vec<String>,
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    /// Retrieve all recipes for a user account.
    pub async fn find_all(
        pool: &sqlx::Pool<sqlx::Postgres>,
        account_id: i32,
    ) -> Result<Vec<Recipe>, sqlx::Error> {
        // ARRAY_AGG allows ingredients to aggregated into an array of tuples.
        sqlx::query_as(
            r#"
			SELECT 
				recipe.id,
				recipe.title,
				recipe.portions,
				recipe.steps,
				ARRAY_AGG((
					ingredient.id,
					ingredient.account_id,
					ingredient.name, 
					ingredient.unit, 
					ingredient.purchase_unit,
					recipe_ingredient.quantity)) as "ingredients"
			
			FROM recipe 
			LEFT JOIN recipe_ingredient ON recipe.id = recipe_ingredient.recipe_id
			LEFT JOIN ingredient ON ingredient.id = recipe_ingredient.ingredient_id

			WHERE recipe.account_id = $1
			GROUP BY recipe.id, recipe.title, recipe.portions, recipe.steps

		"#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await
    }

    /// Retrieve a user account with a given ID.
    pub async fn find_one(
        pool: &sqlx::Pool<sqlx::Postgres>,
        account_id: i32,
        recipe_id: i32,
    ) -> Result<Recipe, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				recipe.id,
				recipe.title,
				recipe.portions,
				recipe.steps,
				ARRAY_AGG((
					ingredient.id,
					ingredient.account_id,
					ingredient.name, 
					ingredient.unit, 
					ingredient.purchase_unit,
					recipe_ingredient.quantity)) as "ingredients"
			
			FROM recipe 
			LEFT JOIN recipe_ingredient ON recipe.id = recipe_ingredient.recipe_id
			LEFT JOIN ingredient ON ingredient.id = recipe_ingredient.ingredient_id

			WHERE recipe.account_id = $1 AND recipe.id = $2
			GROUP BY recipe.id, recipe.title, recipe.portions, recipe.steps

		"#,
        )
        .bind(account_id)
        .bind(recipe_id)
        .fetch_one(pool)
        .await
    }
}
