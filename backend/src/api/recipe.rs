use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::error;

use crate::service::recipe::Recipe;

use super::ServerError;

/// Handler for fetching all recipes for a user.
pub async fn get_recipes(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
) -> Result<Json<Vec<Recipe>>, ServerError> {
    let result = Recipe::find_all(&pool, account_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for fetching a single recipe by ID.
pub async fn get_recipe(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Recipe>, ServerError> {
    let result = Recipe::find_one(&pool, account_id, recipe_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
