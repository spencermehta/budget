use crate::input::{get_float_input, get_input};

#[derive(Debug)]
struct Transaction {
    party: String,
    amount: f64,
}

pub fn create_transaction() -> Transaction {
    println!("Payee:");
    let party = get_input();
    println!("Amount:");
    let amount = get_float_input();

    Transaction { party, amount }
}
