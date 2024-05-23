use super::{recipe::Recipe, utils};

/// Represents a day of a meal plan.
#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct Day {
    pub day_id: i32,
    #[sqlx(flatten)]
    pub recipe: Recipe,
    pub date: chrono::NaiveDate,
}

#[derive(serde::Deserialize, Debug)]
pub struct DayInput {
    recipe_id: i32,
    date: chrono::NaiveDate,
}

impl Day {
    /// Finds all the days for a given user account
    pub async fn find_in_range(
        pool: &sqlx::PgPool,
        account_id: i32,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<Day>, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				day.id as day_id,
				day.date, 
				recipe.id as recipe_id, 
				recipe.title,
				recipe.portions,
				recipe.steps,
				NULL as ingredients
			FROM day
			LEFT JOIN recipe ON recipe.id = recipe_id
			WHERE day.account_id = $1 AND day.date BETWEEN $2 AND $3
			ORDER BY day.date

		"#,
        )
        .bind(account_id)
        .bind(from)
        .bind(to)
        .fetch_all(pool)
        .await
    }

    pub async fn find_one(
        pool: &sqlx::PgPool,
        account_id: i32,
        day_id: i32,
    ) -> Result<Day, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				day.id as day_id,
				day.date, 
				recipe.id as recipe_id, 
				recipe.title,
				recipe.portions,
				recipe.steps,
				NULL as ingredients
			FROM day
			LEFT JOIN recipe ON recipe.id = recipe_id
			WHERE day.account_id = $1 AND day.id = $2
		"#,
        )
        .bind(account_id)
        .bind(day_id)
        .fetch_one(pool)
        .await
    }

    // Creates a new scheduled recipe.
    pub async fn create(
        pool: &sqlx::PgPool,
        account_id: i32,
        input: DayInput,
    ) -> Result<Day, sqlx::Error> {
        let (id,): (i32,) = sqlx::query_as(
            r#"
			INSERT INTO day (account_id, recipe_id, date)
			VALUES ($1, $2, $3)
			RETURNING day.id
			"#,
        )
        .bind(account_id)
        .bind(input.recipe_id)
        .bind(input.date)
        .fetch_one(pool)
        .await?;
        Self::find_one(pool, account_id, id).await
    }

    // Deletes a scheduled recipe.
    pub async fn delete(
        pool: &sqlx::PgPool,
        account_id: i32,
        day_id: i32,
    ) -> Result<(), sqlx::Error> {
        utils::delete_entity(pool, utils::UserDeletable::Day, account_id, day_id).await
    }
}
