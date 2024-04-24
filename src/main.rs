mod category;
mod error;
mod repository;
mod transaction;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use category::{Category, CategoryAssignment, CategoryExpenditureInput};
use error::ApiError;
use repository::Repository;
use std::sync::Arc;
use transaction::Transaction;

struct AppState {
    repository: Repository,
}

#[tokio::main]
async fn main() {
    let repository = Repository::new().await;
    let shared_state = Arc::new(AppState { repository });
    let app = Router::new()
        .route("/transaction/:budget_id", get(get_transactions))
        .route("/transaction", post(create_transaction))
        .route("/category/:budget_id", get(get_categories))
        .route("/category", post(assign_to_category))
        .route("/category/expenditure", post(get_category_expenditure))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_transactions(
    Path(budget_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Transaction>>, ApiError> {
    match state.repository.find_transactions(budget_id).await {
        Ok(txns) => Ok(Json(txns)),
        Err(e) => Err(ApiError::Error(e.to_string())),
    }
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Transaction>,
) -> StatusCode {
    if let Ok(_) = state.repository.insert_transaction(payload).await {
        StatusCode::CREATED
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn get_categories(
    Path(budget_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<String>>) {
    if let Ok(categories) = state.repository.list_categories(budget_id).await {
        (StatusCode::OK, Json(categories))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
    }
}

async fn get_category_expenditure(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CategoryExpenditureInput>,
) -> Result<Json<Vec<Category>>, ApiError> {
    match state
        .repository
        .category_spends(payload.budget_id, payload.date)
        .await
    {
        Ok(categories) => Ok(Json(categories)),
        Err(e) => Err(ApiError::Error(e.to_string())),
    }
}

async fn assign_to_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CategoryAssignment>,
) -> StatusCode {
    if let Ok(_) = state.repository.assign_to_category(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
