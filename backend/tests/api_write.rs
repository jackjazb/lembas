use std::error::Error;

use axum::body::HttpBody;
use axum::http::{self, StatusCode};
use axum::{body::Body, http::Request};
use hyper::body::Buf;
use lembas::{
    api::{day, db::Db, ingredient, recipe},
    router,
};
use sqlx::{Pool, Postgres};
use tower::util::ServiceExt;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn delete_recipe(pool: Pool<Postgres>) -> Result<()> {
    let response = write_resource("DELETE", "recipe/1", &pool, None, None).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);

    let recipes = recipe::query_multiple(&pool, 1)
        .await?
        .iter()
        .map(|r| r.id)
        .collect::<Vec<i32>>();
    assert!(
        !recipes.contains(&1),
        "the database should no longer contain the deleted recipe"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn delete_user_ingredient(pool: Pool<Postgres>) -> Result<()> {
    let response = write_resource("DELETE", "ingredient/2004", &pool, None, None).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);

    let ingredients = ingredient::query_multiple(&pool, 1)
        .await?
        .iter()
        .map(|i| i.id)
        .collect::<Vec<i32>>();
    assert!(
        !ingredients.contains(&2004),
        "the database should no longer contain the deleted ingredient"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn delete_system_ingredient(pool: Pool<Postgres>) -> Result<()> {
    let response = write_resource("DELETE", "ingredient/2000", &pool, None, None).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);

    let ingredients = ingredient::query_multiple(&pool, 1)
        .await?
        .iter()
        .map(|i| i.id)
        .collect::<Vec<i32>>();
    assert!(
        ingredients.contains(&2000),
        "users should not be able to delete system ingredients"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe", "day"))]
fn delete_day_recipe(pool: Pool<Postgres>) -> Result<()> {
    let response = write_resource("DELETE", "days/2023-11-09/recipes/1", &pool, None, None).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);

    let day = &day::query_range(
        &pool,
        1,
        &"2023-11-02".to_string(),
        &"2023-11-09".to_string(),
    )
    .await?;

    let day_recipes = day
        .get(0)
        .unwrap()
        .recipes
        .iter()
        .map(|r| r.id)
        .collect::<Vec<i32>>();

    assert!(
        !day_recipes.contains(&1),
        "the recipe should no longer be present on the specified day"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "ingredientschedule"))]
fn delete_scheduled_ingredient(pool: Pool<Postgres>) -> Result<()> {
    let response = write_resource("DELETE", "schedule/1", &pool, None, None).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);
    let scheduled = ingredient::query_scheduled(&pool, 1)
        .await?
        .iter()
        .map(|si| si.id)
        .collect::<Vec<i32>>();

    assert!(
        !scheduled.contains(&1),
        "the reminder should no longer be present"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient"))]
fn create_recipe(pool: Pool<Postgres>) -> Result<()> {
    let input = include_str!("api/inputs/post-recipe.json").to_string();
    let response = write_resource("POST", "recipes", &pool, None, Some(input.into())).await?;
    assert_eq!(response, StatusCode::CREATED);

    let recipes = recipe::query_multiple(&pool, 1)
        .await?
        .iter()
        .map(|r| r.name.clone())
        .collect::<Vec<String>>();

    assert!(
        recipes.contains(&"Test Recipe".to_string()),
        "the recipe should have been created"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn create_ingredient(pool: Pool<Postgres>) -> Result<()> {
    let input = include_str!("api/inputs/ingredient.json").to_string();
    let response = write_resource("POST", "ingredients", &pool, None, Some(input.into())).await?;
    assert_eq!(response, StatusCode::CREATED);

    let ingredients = ingredient::query_multiple(&pool, 1)
        .await?
        .iter()
        .map(|i| i.name.clone())
        .collect::<Vec<String>>();
    assert!(
        ingredients.contains(&String::from("New Ingredient")),
        "the ingredient should have been created"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn create_day(pool: Pool<Postgres>) -> Result<()> {
    let input = include_str!("api/inputs/day.json").to_string();
    let response = write_resource("POST", "days", &pool, None, Some(input.into())).await?;
    assert_eq!(response, StatusCode::CREATED);

    let day = &day::query_range(
        &pool,
        1,
        &"2023-06-15".to_string(),
        &"2023-06-20".to_string(),
    )
    .await?;
    println!("{:?}", day);
    let day_recipes = day
        .get(0)
        .unwrap()
        .recipes
        .iter()
        .map(|r| r.id)
        .collect::<Vec<i32>>();

    assert!(
        day_recipes.contains(&2),
        "a recipe should be present on the specified date"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient"))]
fn create_schedule(pool: Pool<Postgres>) -> Result<()> {
    let input = include_str!("api/inputs/scheduledingredient.json").to_string();
    let response = write_resource("POST", "schedule", &pool, None, Some(input.into())).await?;
    assert_eq!(response, StatusCode::CREATED);

    let scheduled = ingredient::query_scheduled(&pool, 1)
        .await?
        .iter()
        .map(|si| si.ingredient.id)
        .collect::<Vec<i32>>();

    assert!(
        scheduled.contains(&2004),
        "the reminder should have been created"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn update_recipe(pool: Pool<Postgres>) -> Result<()> {
    let input = include_str!("api/inputs/put-recipe.json").to_string();
    let response = write_resource("PUT", "recipe/2", &pool, None, Some(input.into())).await?;
    assert_eq!(response, StatusCode::NO_CONTENT);

    let recipe = recipe::query_multiple(&pool, 1)
        .await?
        .iter()
        .filter(|r| r.id == 2)
        .map(|r| r.name.clone())
        .collect::<Vec<String>>();
    let name = recipe.get(0).unwrap();

    assert!(
        name == &"Renamed".to_string(),
        "the recipe should have been renamed"
    );
    Ok(())
}

/// Hits an API route with the specified method and verified the response
async fn write_resource(
    method: &str,
    resource: &str,
    pool: &Pool<Postgres>,
    params: Option<&str>,
    body: Option<Body>,
) -> Result<StatusCode> {
    let db = Db { pool: pool.clone() };
    let router = router::make_router(db, String::from("localhost"), false).await?;

    // Construct URI.
    let path = match params {
        Some(str) => format!("/{}?{}", resource, str),
        None => format!("/{}", resource),
    };
    let uri = http::uri::Builder::new().path_and_query(path).build()?;
    let body = body.unwrap_or(Body::empty());

    println!("{:#?}", body);

    let mut response = router
        .oneshot(
            Request::builder()
                .method(method)
                .header("Authorization", "Bearer abc")
                .header("Content-Type", "application/json")
                .uri(uri)
                .body(body)?,
        )
        .await?;
    // Get response body as JSON.
    let data = response.body_mut().data().await.unwrap_or(Ok("".into()))?;

    // This will print the response if the test fails.
    println!(
        "Response: {:#?}",
        &String::from_utf8_lossy(data.chunk()).to_string()
    );
    Ok(response.status())
}
