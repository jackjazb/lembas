use std::{env, error::Error};

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};

use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::{
    api::{
        account::{get_account, get_accounts},
        ingredient::{create_ingredient, get_ingredient},
        recipe::{create_recipe, get_recipe, get_recipes, update_recipe},
        reminder::{create_reminder, get_reminder, get_reminders},
    },
    service::devdata::load_data,
};

pub type ServerError = (StatusCode, String);

#[tokio::main]
pub async fn start() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Load environment.
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let env = env::var("ENV").expect("ENV is not set in .env file");
    let server_url = format!("{host}:{port}");

    // Connect to database and run migrations.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    sqlx::migrate!().run(&pool).await?;
    info!("connected to database at {}", db_url);

    // Optionally load development data.
    if env == "dev" {
        load_data(&pool).await?;
        info!("loaded development data")
    }

    // Set up the main app router.
    let app = axum::Router::new()
        .route("/", get(health_check))
        .route("/accounts", get(get_accounts))
        .route("/accounts/:id", get(get_account))
        .route("/recipes", get(get_recipes).post(create_recipe))
        .route("/recipes/:id", get(get_recipe).post(update_recipe))
        .route("/reminders", get(get_reminders).post(create_reminder))
        .route("/reminders/:id", get(get_reminder))
        .route("/ingredients", post(create_ingredient))
        .route("/ingredients/:id", get(get_ingredient))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    info!("server listening on {}", server_url);

    axum::serve(listener, app).await?;
    Ok(())
}

/// Serves as a basic health check
async fn health_check() -> String {
    "server up".to_string()
}

async fn auth_middleware(mut request: Request, next: Next) -> Response {
    request.extensions_mut().insert(1);
    next.run(request).await
}
