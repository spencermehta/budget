use crate::input::{get_float_input, get_input};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub party: String,
    pub category: String,
    pub amount: f64,
}

pub fn create_transaction() -> Transaction {
    println!("Payee:");
    let party = get_input();
    println!("Category:");
    let category = get_input();
    println!("Amount:");
    let amount = get_float_input();

    Transaction {
        party,
        category,
        amount,
    }
}
