mod category;
mod input;
mod mongo_repository;
mod repository;
mod transaction;

use input::get_input;
use repository::Repository;
use transaction::create_transaction;

fn main() {
    let repository = Repository::new();
    get_user_input(repository);
}

fn get_user_input(repository: Repository) {
    loop {
        println!("\nSelect an option:\nq: Quit\n1: Add transaction\n2: Show transactions\n3: List categories");
        let choice = get_input();
        match choice.as_str() {
            "q" => break,
            "1" => {
                insert_transaction(&repository);
            }
            "2" => print_transactions(&repository),
            "3" => print_categories(&repository),
            _ => {}
        }
    }
}

fn insert_transaction(repository: &Repository) {
    let transaction = create_transaction();
    let _ = repository.insert_transaction(transaction);
}

fn print_transactions(repository: &Repository) {
    let txns = repository.find_transaction().unwrap();

    for txn in txns {
        println!("{:?}", txn)
    }
}

fn print_categories(repository: &Repository) {
    let categories = repository.list_categories().unwrap();
    for category in categories {
        println!("{}", category);
    }
}
