use crate::input;
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

#[derive(Debug, Serialize, Deserialize)]
struct SpentAggregate {
    spent: f64,
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

    pub fn get_available_to_budget(&self) -> mongodb::error::Result<f64> {
        let pipeline =
            vec![doc! {"$group": doc! {"_id": "sum", "spent": doc! {"$sum": "$amount"}}}];
        let mut results = self.transactions.aggregate(pipeline, None)?;
        if let Some(Ok(sum)) = results.next() {
            let doc = bson::from_document::<SpentAggregate>(sum);
            Ok(doc.unwrap().spent)
        } else {
            Ok(0.0)
        }
    }
}

pub fn create_transaction() -> Transaction {
    println!("Payee:");
    let party = input::get_input();
    println!("Category:");
    let category = input::get_input();
    println!("Amount:");
    let amount = input::get_float_input();

    Transaction {
        date: Utc::now(),
        party,
        category,
        amount,
    }
}
