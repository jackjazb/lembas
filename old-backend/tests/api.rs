use std::error::Error;

use axum::body::HttpBody;
use axum::http::{self, StatusCode};
use axum::{body::Body, http::Request};
use hyper::body::Buf;
use lembas::{api::db::Db, router};
use sqlx::{Pool, Postgres};
use tower::util::ServiceExt;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

// Recipe CRUD endpoints

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn get_recipes(pool: Pool<Postgres>) -> Result<()> {
    test_route_response("GET", "recipes", &pool, None).await?;
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn get_user_ingredients(pool: Pool<Postgres>) -> Result<()> {
    test_route_response("GET", "ingredients/user", &pool, None).await?;
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn get_recipe(pool: Pool<Postgres>) -> Result<()> {
    test_route_response("GET", "recipe/1", &pool, None).await?;
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe", "day"))]
fn get_days(pool: Pool<Postgres>) -> Result<()> {
    test_route_response("GET", "days", &pool, Some("from=2023-11-01&to=2023-11-10")).await?;
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe", "ingredientschedule"))]
fn get_schedule(pool: Pool<Postgres>) -> Result<()> {
    test_route_response("GET", "schedule", &pool, None).await?;
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe", "day", "ingredientschedule"))]
fn get_shopping_list(pool: Pool<Postgres>) -> Result<()> {
    test_route_response(
        "GET",
        "shoppinglist",
        &pool,
        Some("from=2023-11-01&to=2023-11-10"),
    )
    .await?;
    Ok(())
}

/// Hits an API route with the specified method and verified the response
async fn test_route_response(
    method: &str,
    resource: &str,
    pool: &Pool<Postgres>,
    params: Option<&str>,
) -> Result<()> {
    let db = Db { pool: pool.clone() };
    let router = router::make_router(db, String::from("localhost"), false).await?;

    // Construct URI.
    let path = match params {
        Some(str) => format!("/{}?{}", resource, str),
        None => format!("/{}", resource),
    };
    let uri = http::uri::Builder::new().path_and_query(path).build()?;

    let mut response = router
        .oneshot(
            Request::builder()
                .method(method)
                .header("Authorization", "Bearer abc")
                .uri(uri)
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "response should have status 200"
    );

    // Get response body as JSON.
    let data = response.body_mut().data().await.unwrap()?;

    // This will print the response if the test fails.
    println!(
        "Response: {:#?}",
        &String::from_utf8_lossy(data.chunk()).to_string()
    );

    let json: serde_json::Value =
        serde_json::from_str(&String::from_utf8_lossy(data.chunk()).to_string())?;

    let path = resource.to_lowercase().replace("/", "-");

    // Read expected response as JSON.
    let response_path = format!(
        "./tests/api/responses/{}-{}.json",
        method.to_lowercase(),
        path
    );
    let expected: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(response_path)?)?;

    assert_eq!(
        expected,
        json,
        "{}",
        format!(
            "response should match the expected output in {}.json",
            resource.to_lowercase()
        )
    );
    Ok(())
}
