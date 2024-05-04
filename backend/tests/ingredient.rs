mod common;

use crate::common::test_ingredients;
use lembas::api::ingredient;
use sqlx::{Pool, Postgres};

#[sqlx::test(fixtures("useraccount", "ingredient"))]
fn search_ingredients(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let query: String = "Fl".into();
    let mut results = ingredient::search(&pool, 1, query).await?;
    let first = results.pop().unwrap();
    assert_eq!(first.name, "Flour", "query should have returned flour");
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient"))]
fn query_all_ingredients(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let results = ingredient::query_multiple(&pool, 1).await?;
    let carrot = test_ingredients().get("Carrot".into()).unwrap().clone();
    let water = test_ingredients().get("Water".into()).unwrap().clone();

    assert!(results.contains(&carrot), "results should contain carrot");
    assert!(results.contains(&water), "results should contain water");
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "ingredientschedule"))]
fn query_schedule(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let results = ingredient::query_scheduled(&pool, 1).await?;
    let carrot = test_ingredients().get("Carrot".into()).unwrap().clone();
    let scheduled_carrot = ingredient::ScheduledIngredient {
        id: 1,
        ingredient: carrot,
        start_date: "2023-11-05".into(),
        interval: 7,
    };

    assert_eq!(results.len(), 1, "should be one result");
    assert_eq!(
        results,
        vec![scheduled_carrot],
        "results should contain a schedule for carrots"
    );
    Ok(())
}
