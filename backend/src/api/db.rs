use std::{env, error::Error};

use sqlx::migrate::MigrateError;

/// Encapsulates all state required for accessing the database.
#[derive(Clone)]
pub struct Db {
    pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl Db {
    /// Create a new database connection.
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        if let Ok(url) = env::var("DATABASE_URL") {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&url)
                .await?;
            return Ok(Db { pool });
        }

        return Err("DATABASE_URL must be set")?;
    }

    /// Run the SQLx database migrations on this database instance.
    pub async fn migrate(&self) -> Result<(), MigrateError> {
        sqlx::migrate!().run(&self.pool).await
    }
}
