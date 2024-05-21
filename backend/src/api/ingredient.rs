use crate::service::ingredient::{Ingredient, IngredientInput};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::error;

use super::router::ServerError;

/// Handler for fetching all recipes for a user.
pub async fn get_ingredient(
    State(pool): State<sqlx::PgPool>,
    Path(ingredient_id): Path<i32>,
    Extension(account_id): Extension<i32>,
) -> Result<Json<Ingredient>, ServerError> {
    let result = Ingredient::find_one(&pool, account_id, ingredient_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn create_ingredient(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Json(ingredient): Json<IngredientInput>,
) -> Result<Json<Ingredient>, ServerError> {
    let result = Ingredient::create(&pool, account_id, ingredient).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
