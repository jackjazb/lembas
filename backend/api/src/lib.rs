use async_graphql_axum::GraphQL;
use std::{env, error::Error};

use axum::{response, routing::get};
use entity::{
    async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema},
    recipe,
    sea_orm::{Database, DbConn, DbErr, EntityTrait},
};
use migration::{Migrator, MigratorTrait};
use tracing::info;

use crate::schema::QueryRoot;

mod schema;

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
    let server_url = format!("{host}:{port}");

    // Set up database connection and run migrations.
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    // Build GraphQL Schema.
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(conn) // Add the database connection to the GraphQL global context
        .finish();

    // Set up the main app router.
    let app = axum::Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    info!("server listening on {}", server_url);

    axum::serve(listener, app).await?;
    Ok(())
}

/// Serves as a basic health check
async fn health_check() -> String {
    "server up".to_string()
}

/// Serves a GraphQL UI.
async fn graphiql() -> impl response::IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn find_recipes_for_user(
    db: &DbConn,
    user_id: i32,
) -> Result<Option<recipe::Model>, DbErr> {
    recipe::Entity::find_by_id(user_id).one(db).await
}
