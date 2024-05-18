mod common;

use lembas::api::day::{self, DayInput};
use sqlx::{Pool, Postgres};

/// Sets up a recipe that uses 5 carrots (half of the minimum purchase)
/// on January 10th 2020.
async fn setup_meal_plan(pool: &Pool<Postgres>) -> sqlx::Result<()> {
    // Make the recipe on Jan 10th.
    let day_input = DayInput {
        recipe_id: 1,
        date: "2020-01-10".into(),
    };
    day::create(&pool, 1, day_input).await.unwrap();

    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn generate_list_without_surplus(pool: Pool<Postgres>) -> sqlx::Result<()> {
    setup_meal_plan(&pool).await?;
    let list = day::build_list_for_range(&pool, 1, "2020-01-04".into(), "2020-01-11".into())
        .await
        .unwrap();
    let ingredient = list.ingredients.get(0).unwrap();

    assert_eq!(ingredient.used_quantity, 5);
    assert_eq!(
        ingredient.purchase_quantity, 10,
        "should have added the minimum purchase quantity to the list"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn generate_list_with_exact_previous_surplus(pool: Pool<Postgres>) -> sqlx::Result<()> {
    setup_meal_plan(&pool).await?;

    // Make the recipe again 5 days later.
    let later_day_input = DayInput {
        recipe_id: 1,
        date: "2020-01-15".into(),
    };
    day::create(&pool, 1, later_day_input).await.unwrap();

    /* The list should:
      - Find the usage of carrots on the 10th
      - Identify 5 are left over, which covers the amount needed
      - Not suggest any more should be purchased
    */
    let list = day::build_list_for_range(&pool, 1, "2020-01-11".into(), "2020-01-16".into())
        .await
        .unwrap();
    let ingredient = list.ingredients.get(0).unwrap();

    assert_eq!(ingredient.used_quantity, 5);
    assert_eq!(
        ingredient.purchase_quantity, 0,
        "should have removed the existing surplus from the purchase quantity"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn generate_list_with_insufficient_previous_surplus(pool: Pool<Postgres>) -> sqlx::Result<()> {
    setup_meal_plan(&pool).await?;

    // Make the recipe again 5 days later.
    let later_day_input = DayInput {
        recipe_id: 2,
        date: "2020-01-15".into(),
    };
    day::create(&pool, 1, later_day_input).await.unwrap();

    /* The list should:
      - Find the usage of carrots on the 10th
      - Identify 5 are left over, which only covers 5/6
      - Suggest an additional minimum quantity be purchased
    */
    let list = day::build_list_for_range(&pool, 1, "2020-01-11".into(), "2020-01-16".into())
        .await
        .unwrap();
    let ingredient = list.ingredients.get(0).unwrap();

    assert_eq!(ingredient.used_quantity, 6);
    assert_eq!(
        ingredient.purchase_quantity, 10,
        "should have removed the existing surplus from the purchase quantity"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn generate_list_with_expired_surplus(pool: Pool<Postgres>) -> sqlx::Result<()> {
    setup_meal_plan(&pool).await?;

    // Make the recipe again 20 days later.
    let later_day_input = DayInput {
        recipe_id: 1,
        date: "2020-01-30".into(),
    };
    day::create(&pool, 1, later_day_input).await.unwrap();

    /* The list should:
      - Ignore the usage of carrots on the 10th
      - Suggest an additional purchase
    */
    let list = day::build_list_for_range(&pool, 1, "2020-01-29".into(), "2020-02-01".into())
        .await
        .unwrap();
    let ingredient = list.ingredients.get(0).unwrap();

    assert_eq!(ingredient.used_quantity, 5);
    assert_eq!(
        ingredient.purchase_quantity, 10,
        "should have excluded expired ingredients from the surplus"
    );
    Ok(())
}
