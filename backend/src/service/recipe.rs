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

/// Allows the creation of recipes.
/// Note that the members of each ingredient are an ID and quantity respectively.
#[derive(serde::Deserialize, Debug)]
pub struct RecipeInput {
    title: String,
    portions: i32,
    steps: Vec<String>,
    ingredients: Vec<(i32, f64)>,
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
					ingredient.life,
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

    /// Retrieve a recipe with a given ID.
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
					ingredient.life,
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

    /// Create a new recipe.
    pub async fn create(
        pool: &sqlx::PgPool,
        account_id: i32,
        input: RecipeInput,
    ) -> Result<Recipe, sqlx::Error> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

        // Create a new entry in `recipes`.
        let (recipe_id,): (i32,) = sqlx::query_as(
            r#"
			INSERT INTO recipe (account_id, title, portions, steps)
			VALUES ($1, $2, $3, $4)
			RETURNING recipe.id
			"#,
        )
        .bind(account_id)
        .bind(input.title)
        .bind(input.portions)
        .bind(input.steps)
        .fetch_one(&mut **(&mut tx))
        .await?;

        // Create entries for the ingredients in `recipe_ingredient`.
        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            "INSERT INTO recipe_ingredient (recipe_id, ingredient_id, quantity)",
        );
        qb.push_values(input.ingredients, |mut b, (id, quantity)| {
            b.push_bind(recipe_id).push_bind(id).push_bind(quantity);
        });

        let query = qb.build();
        query.execute(&mut **(&mut tx)).await?;
        tx.commit().await?;

        // Return the newly created recipe.
        Self::find_one(pool, account_id, recipe_id).await
    }

    /// Update a recipe.
    pub async fn update(
        pool: &sqlx::PgPool,
        account_id: i32,
        recipe_id: i32,
        input: RecipeInput,
    ) -> Result<Recipe, sqlx::Error> {
        let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

        // The easiest way of maintaining data integrity here is to wipe and recreate.
        sqlx::query(
            r#"
			DELETE FROM recipe WHERE id = $1
		"#,
        )
        .bind(recipe_id)
        .execute(&mut **(&mut tx))
        .await?;

        // Recreate the recipe.
        sqlx::query(
            r#"
			INSERT INTO recipe (id, account_id, title, portions, steps)
			VALUES ($1, $2, $3, $4, $5)
			RETURNING recipe.id
			"#,
        )
        .bind(recipe_id)
        .bind(account_id)
        .bind(input.title)
        .bind(input.portions)
        .bind(input.steps)
        .execute(&mut **(&mut tx))
        .await?;

        let mut qb: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
            "INSERT INTO recipe_ingredient (recipe_id, ingredient_id, quantity)",
        );
        qb.push_values(input.ingredients, |mut b, (id, quantity)| {
            b.push_bind(recipe_id).push_bind(id).push_bind(quantity);
        });

        let query = qb.build();
        query.execute(&mut **(&mut tx)).await?;
        tx.commit().await?;

        // Return the updated recipe
        Self::find_one(pool, account_id, recipe_id).await
    }
}
