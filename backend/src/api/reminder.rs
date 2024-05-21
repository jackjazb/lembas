use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use tracing::error;

use crate::service::reminder::{Reminder, ReminderInput};

use super::router::ServerError;

/// Handler for fetching all recipes for a user.
pub async fn get_reminders(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
) -> Result<Json<Vec<Reminder>>, ServerError> {
    let result = Reminder::find_all(&pool, account_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for fetching all recipes for a user.
pub async fn get_reminder(
    State(pool): State<sqlx::PgPool>,
    Path(reminder_id): Path<i32>,
    Extension(account_id): Extension<i32>,
) -> Result<Json<Reminder>, ServerError> {
    let result = Reminder::find_one(&pool, account_id, reminder_id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn create_reminder(
    State(pool): State<sqlx::PgPool>,
    Extension(account_id): Extension<i32>,
    Json(reminder): Json<ReminderInput>,
) -> Result<Json<Reminder>, ServerError> {
    let result = Reminder::create(&pool, account_id, reminder).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
