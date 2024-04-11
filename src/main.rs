mod category;
mod input;
mod mongo_repository;
mod repository;
mod transaction;

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
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
        .route("/", get(root))
        .route("/transactions", get(get_transactions))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, world!"
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

async fn get_user_input(repository: Repository) -> mongodb::error::Result<()> {
    loop {
        let available = repository.get_available_to_budget().await?;
        println!("You have {} left to budget.", available);
        println!("\nSelect an option:\nq: Quit\n1: Add transaction\n2: Show transactions\n3: List categories\n4: Show expenditure\n5: Set budget");
        let choice = input::get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                insert_transaction(&repository);
            }
            "2" => print_transactions(&repository).await?,
            "3" => print_categories(&repository).await?,
            "4" => print_category_expenditure(&repository).await?,
            "5" => set_budget_for_category(&repository),
            _ => {}
        }
    }

    Ok(())
}

fn insert_transaction(repository: &Repository) {
    let transaction = transaction::create_transaction();
    let _ = repository.insert_transaction(transaction);
}

async fn print_transactions(repository: &Repository) -> mongodb::error::Result<()> {
    let txns = repository.find_transaction().await?;

    for txn in txns {
        println!("{:?}", txn)
    }

    Ok(())
}

async fn print_categories(repository: &Repository) -> mongodb::error::Result<()> {
    let categories = repository.list_categories().await?;
    for category in categories {
        println!("{}", category);
    }
    Ok(())
}

async fn print_category_expenditure(repository: &Repository) -> mongodb::error::Result<()> {
    let categories = repository.category_spends().await?;
    for category in categories {
        println!("{:?}", category);
    }
    Ok(())
}

fn set_budget_for_category(repository: &Repository) {
    let category = category::create_category();
    let _ = repository.set_budget_for_category(category);
}
