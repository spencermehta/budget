mod category;
mod input;
mod mongo_repository;
mod repository;
mod transaction;

use category::{create_category, Category};
use input::get_input;
use repository::Repository;
use transaction::{create_transaction, Transaction};

fn main() {
    let repository = Repository::new();
    let _ = repository.find_transaction();
}

fn get_user_input() {
    let mut categories: Vec<Category> = Vec::new();
    let mut transactions: Vec<Transaction> = Vec::new();

    loop {
        println!("\nSelect an option:\nq: Quit\n1: Create category\n2: Add transaction\n3: Show categories\n4: Show transactions\n");
        let choice = get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                let category = create_category();
                println!("Created category {:?}", category);
                categories.push(category);
            }
            "2" => {
                let transaction = create_transaction();
                println!("Created transaction {:?}", transaction);
                transactions.push(transaction);
            }
            "3" => {
                let mut sum: f64 = 0.0;
                for category in &categories {
                    for transaction in &transactions {
                        if transaction.category == category.name {
                            sum += transaction.amount;
                        }
                    }
                    println!("{:?}: spent {}", category, sum);
                }
            }
            "4" => {
                for transaction in &transactions {
                    println!("{:?}", transaction);
                }
            }
            _ => {}
        }
    }
}
