mod category;
mod input;
mod transaction;

use category::{create_category, Category};
use input::get_input;
use transaction::{create_transaction, Transaction};

fn main() {
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
                for category in &categories {
                    println!("{:?}", category);
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
