mod common;

use lembas::api::{ingredient, recipe};
use sqlx::{Pool, Postgres};

use crate::common::test_ingredients;

#[sqlx::test(fixtures("useraccount", "ingredient"))]
fn create_and_fetch_recipe(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let flour = test_ingredients().get("Flour".into()).unwrap().clone();

    // Expecting 1000g of Flour
    let expected_ingredient_quantities =
        vec![ingredient::IngredientQuantity::new(flour.clone(), 1000)];
    let expected_steps = sqlx::types::JsonValue::Array(vec!["step one".into(), "step two".into()]);

    // Perform recipe insertion
    let ingredient_inputs = vec![recipe::RecipeIngredientInput {
        id: 2,
        quantity: 1000,
    }];
    let recipe_input = recipe::RecipeInput {
        name: "Test Recipe".into(),
        portions: 4,
        steps: sqlx::types::JsonValue::Array(vec!["step one".into(), "step two".into()]),
        ingredients: ingredient_inputs,
    };

    let id = recipe::create(&pool, 1, recipe_input).await?;

    // Fetch all recipes and verify expected insertion took place
    let recipes = recipe::query_multiple(&pool, 1).await?;
    assert_eq!(
        recipes,
        vec![recipe::Recipe::new(
            id,
            "Test Recipe".into(),
            4,
            expected_steps,
            expected_ingredient_quantities
        )],
        "a recipe should have been created and then retrieved"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn delete_recipe(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let count_before_deletion = recipe::query_multiple(&pool, 1).await?.len();

    // Delete recipe
    recipe::delete(&pool, 1, 1).await?;

    let count_after_deletion = recipe::query_multiple(&pool, 1).await?.len();
    assert_eq!(
        count_before_deletion - count_after_deletion,
        1,
        "a recipe should have been deleted"
    );
    Ok(())
}

#[sqlx::test(fixtures("useraccount", "ingredient", "recipe"))]
fn query_single_recipe(pool: Pool<Postgres>) -> sqlx::Result<()> {
    let recipe = recipe::query_single(&pool, 1, 1).await?;
    let carrot = test_ingredients().get("Carrot".into()).unwrap().clone();

    let expected_ingredients = vec![ingredient::IngredientQuantity::new(carrot, 5)];
    assert_eq!(
        recipe.ingredients, expected_ingredients,
        "a single recipe should have been fetched"
    );
    Ok(())
}
