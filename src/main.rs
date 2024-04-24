mod category;
mod error;
mod repository;
mod transaction;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use category::{BudgetCategory, Category};
use chrono::Utc;
use error::ApiError;
use repository::Repository;
use std::sync::Arc;
use transaction::{CreateTransaction, Transaction};

struct AppState {
    repository: Repository,
}

#[tokio::main]
async fn main() {
    let repository = Repository::new().await;
    let shared_state = Arc::new(AppState { repository });
    let app = Router::new()
        .route(
            "/transaction",
            get(get_transactions).post(create_transaction),
        )
        .route(
            "/category",
            get(get_categories).post(set_budget_for_category),
        )
        .route("/category/expenditure", get(get_category_expenditure))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_transactions(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, Json<Vec<Transaction>>) {
    if let Ok(txns) = state.repository.find_transaction().await {
        (StatusCode::OK, Json(txns))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
    }
}

async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTransaction>,
) -> StatusCode {
    let txn = Transaction {
        date: Utc::now(),
        party: payload.party,
        category: payload.category,
        amount: payload.amount,
    };

    if let Ok(_) = state.repository.insert_transaction(txn).await {
        StatusCode::CREATED
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

async fn get_categories(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Vec<String>>) {
    if let Ok(categories) = state.repository.list_categories().await {
        (StatusCode::OK, Json(categories))
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new()))
    }
}

async fn get_category_expenditure(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Category>>, ApiError> {
    match state.repository.category_spends().await {
        Ok(categories) => Ok(Json(categories)),
        Err(e) => Err(ApiError::Error(e.to_string())),
    }
}

async fn set_budget_for_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BudgetCategory>,
) -> StatusCode {
    if let Ok(_) = state.repository.set_budget_for_category(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
