use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::error;

use crate::service::day::{Day, DayInput};

use super::router::ServerError;

#[derive(serde::Deserialize)]
pub struct DayRangeParams {
    pub from: chrono::NaiveDate,
    pub to: chrono::NaiveDate,
}

/// Handler for retrieving scheduled recipes in a date range.
pub async fn get_days(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Query(range): Query<DayRangeParams>,
) -> Result<Json<Vec<Day>>, ServerError> {
    let result = Day::find_in_range(&pool, account_id, range.from, range.to).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for creating a new scheduled recipe.
pub async fn create_day(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Json(input): Json<DayInput>,
) -> Result<Json<Day>, ServerError> {
    let result = Day::create(&pool, account_id, input).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for deleting a scheduled recipe.
pub async fn delete_day(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Path(day_id): Path<i32>,
) -> Result<StatusCode, ServerError> {
    let result = Day::delete(&pool, account_id, day_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
