use crate::input::{get_float_input, get_input};
use crate::repository::Repository;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub date: DateTime<Utc>,
    pub party: String,
    pub category: String,
    pub amount: f64,
}

impl Repository {
    pub fn find_transaction(&self) -> mongodb::error::Result<Vec<Transaction>> {
        let cursor = self.transactions.find(doc! {}, None)?;
        let mut txns = Vec::new();
        for txn in cursor {
            match txn {
                Ok(t) => txns.push(t),
                Err(e) => println!("{}", e),
            }
        }

        Ok(txns)
    }

    pub fn insert_transaction(&self, transaction: Transaction) -> mongodb::error::Result<()> {
        let docs = vec![transaction];
        self.transactions.insert_many(docs, None)?;
        Ok(())
    }
}

pub fn create_transaction() -> Transaction {
    println!("Payee:");
    let party = get_input();
    println!("Category:");
    let category = get_input();
    println!("Amount:");
    let amount = get_float_input();

    Transaction {
        date: Utc::now(),
        party,
        category,
        amount,
    }
}
