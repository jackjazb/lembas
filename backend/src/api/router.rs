use std::{env, error::Error, process::exit};

use axum::{
    extract::Request,
    http::{self, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{delete, get, post},
};

use sqlx::postgres::PgPoolOptions;
use tower_http::cors;
use tracing::info;

use crate::{
    api::{
        account::{get_account, get_accounts},
        day::{create_day, delete_day, get_days},
        ingredient::{
            create_ingredient, delete_ingredient, get_ingredient, get_ingredients,
            update_ingredient,
        },
        list_builder::get_list_for_range,
        recipe::{create_recipe, delete_recipe, get_recipe, get_recipes, update_recipe},
        recipe_parser::{parse_handler, parse_recipe},
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

    // Set up CORS.
    let cors = cors::CorsLayer::new()
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::DELETE])
        .allow_headers([http::header::AUTHORIZATION, http::header::CONTENT_TYPE])
        .allow_origin(cors::Any);

    // Set up the main app router.
    let app = axum::Router::new()
        .route("/", get(health_check))
        .route("/accounts", get(get_accounts))
        .route("/accounts/:id", get(get_account))
        .route("/recipes", get(get_recipes).post(create_recipe))
        .route(
            "/recipes/:id",
            get(get_recipe).post(update_recipe).delete(delete_recipe),
        )
        .route("/reminders", get(get_reminders).post(create_reminder))
        .route("/reminders/:id", get(get_reminder))
        .route("/ingredients", post(create_ingredient).get(get_ingredients))
        .route(
            "/ingredients/:id",
            get(get_ingredient)
                .delete(delete_ingredient)
                .post(update_ingredient),
        )
        .route("/days", get(get_days).post(create_day))
        .route("/days/:id", delete(delete_day))
        .route("/list", get(get_list_for_range))
        .route("/parse", get(parse_handler))
        .layer(tower::ServiceBuilder::new().layer(cors))
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
