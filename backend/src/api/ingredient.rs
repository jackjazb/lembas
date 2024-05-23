use crate::service::ingredient::{Ingredient, IngredientInput};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::error;

use super::router::ServerError;

/// Handler for fetching all ingredients.
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

/// Parameters for the search handler.
#[derive(serde::Deserialize)]
pub struct IngredientSearchParams {
    query: Option<String>,
}

/// Handler for searching over ingredients.
pub async fn get_ingredients(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Query(params): Query<IngredientSearchParams>,
) -> Result<Json<Vec<Ingredient>>, ServerError> {
    let result = match params.query {
        Some(text) => Ingredient::search(&pool, account_id, text).await,
        None => Ingredient::find_all(&pool, account_id).await,
    };

    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for creating a custom ingredient.
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

/// Handler for creating a custom ingredient.
pub async fn update_ingredient(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Path(ingredient_id): Path<i32>,
    Json(ingredient): Json<IngredientInput>,
) -> Result<Json<Ingredient>, ServerError> {
    let result = Ingredient::update(&pool, account_id, ingredient_id, ingredient).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for deleting a custom ingredient.
pub async fn delete_ingredient(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Path(ingredient_id): Path<i32>,
) -> Result<StatusCode, ServerError> {
    let result = Ingredient::delete(&pool, account_id, ingredient_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
