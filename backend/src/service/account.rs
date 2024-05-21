/// Represents a user account.
#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct Account {
    id: i32,
    email: String,
}

impl Account {
    /// Retrieve all user acccounts.
    pub async fn find_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Account>, sqlx::Error> {
        sqlx::query_as("SELECT * FROM account")
            .fetch_all(pool)
            .await
    }

    /// Retrieve a user account with a given ID.
    pub async fn find_one(
        pool: &sqlx::Pool<sqlx::Postgres>,
        account_id: i32,
    ) -> Result<Account, sqlx::Error> {
        sqlx::query_as("SELECT * FROM account WHERE id = $1")
            .bind(account_id)
            .fetch_one(pool)
            .await
    }
}
