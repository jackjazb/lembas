use super::ingredient::Ingredient;

/// Represents a reminder to purchase an ingredient at a set interval.
#[derive(serde::Serialize, sqlx::FromRow, Debug)]
pub struct Reminder {
    reminder_id: i32,
    #[sqlx(flatten)]
    ingredient: Ingredient,
    start: chrono::NaiveDate,
    interval: i32,
}

#[derive(serde::Deserialize, Debug)]
pub struct ReminderInput {
    ingredient_id: i32,
    start: chrono::NaiveDate,
    interval: i32,
}

impl Reminder {
    /// Finds all the reminders for a given user account
    pub async fn find_all(
        pool: &sqlx::PgPool,
        account_id: i32,
    ) -> Result<Vec<Reminder>, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				reminder.id as reminder_id,
				reminder.start, 
				reminder.interval, 
				ingredient.id as ingredient_id, 
				ingredient.account_id,
				ingredient.name, 
				ingredient.unit, 
				ingredient.purchase_unit,
				ingredient.life,
				ingredient.purchase_unit as quantity
			FROM reminder
			LEFT JOIN ingredient ON ingredient.id = ingredient_id
			WHERE reminder.account_id = $1
		"#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await
    }

    // Fetches a single reminder by ID.
    pub async fn find_one(
        pool: &sqlx::PgPool,
        account_id: i32,
        reminder_id: i32,
    ) -> Result<Reminder, sqlx::Error> {
        sqlx::query_as(
            r#"
			SELECT 
				reminder.id as reminder_id,
				reminder.start, 
				reminder.interval,
				ingredient.id as ingredient_id, 
				ingredient.account_id,
				ingredient.name, 
				ingredient.unit, 
				ingredient.purchase_unit,
				ingredient.life,
				ingredient.purchase_unit as quantity
			FROM reminder
			LEFT JOIN ingredient ON ingredient.id = ingredient_id
			WHERE reminder.account_id = $1 AND reminder.id = $2
			"#,
        )
        .bind(account_id)
        .bind(reminder_id)
        .fetch_one(pool)
        .await
    }

    // Creates a new reminder.
    pub async fn create(
        pool: &sqlx::PgPool,
        account_id: i32,
        input: ReminderInput,
    ) -> Result<Reminder, sqlx::Error> {
        let (id,): (i32,) = sqlx::query_as(
            r#"
			INSERT INTO reminder (account_id, ingredient_id, start, interval)
			VALUES ($1, $2, $3, $4)
			RETURNING reminder.id
			"#,
        )
        .bind(account_id)
        .bind(input.ingredient_id)
        .bind(input.start)
        .bind(input.interval)
        .fetch_one(pool)
        .await?;
        Self::find_one(pool, account_id, id).await
    }
}
