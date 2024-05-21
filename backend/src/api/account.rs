use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::error;

use crate::service::account::Account;

use super::ServerError;

/// Handler for fetching all user accounts.
pub async fn get_accounts(
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<Vec<Account>>, ServerError> {
    let result = Account::find_all(&pool).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

/// Handler for fetching a single user account by ID.
pub async fn get_account(
    State(pool): State<sqlx::PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Account>, ServerError> {
    let result = Account::find_one(&pool, id).await;
    if result.is_err() {
        error!("{:?}", result);
    }

    result
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
