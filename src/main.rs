mod category;
mod input;
mod mongo_repository;
mod repository;
mod transaction;

use category::create_category;
use input::get_input;
use repository::Repository;
use transaction::create_transaction;

fn main() {
    let repository = Repository::new();
    get_user_input(repository);
}

fn get_user_input(repository: Repository) {
    loop {
        println!("\nSelect an option:\nq: Quit\n1: Create category\n2: Add transaction\n3: Show categories\n4: Show transactions\n");
        let choice = get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                let category = create_category();
                println!("Created category {:?}", category);
            }
            "2" => {
                let transaction = create_transaction();
                println!("Created transaction {:?}", transaction);
            }
            "3" => {
                let mut sum: f64 = 0.0;
                // for category in &categories {
                //     for transaction in &transactions {
                //         if transaction.category == category.name {
                //             sum += transaction.amount;
                //         }
                //     }
                //     println!("{:?}: spent {}", category, sum);
                // }
            }
            "4" => print_transactions(&repository),
            _ => {}
        }
    }
}

fn print_transactions(repository: &Repository) {
    let txns = repository.find_transaction().unwrap();

    for txn in txns {
        println!("{:?}", txn)
    }
}
