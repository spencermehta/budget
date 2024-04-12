use crate::input;
use crate::repository::Repository;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use futures_util::TryStreamExt;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransaction {
    pub party: String,
    pub category: String,
    pub amount: f64,
}

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
    pub async fn find_transaction(&self) -> mongodb::error::Result<Vec<Transaction>> {
        let mut cursor = self.transactions.find(doc! {}, None).await?;
        let mut txns = Vec::new();
        while let Some(txn) = cursor.try_next().await? {
            txns.push(txn);
        }

        Ok(txns)
    }

    pub async fn insert_transaction(&self, transaction: Transaction) -> mongodb::error::Result<()> {
        let docs = vec![transaction];
        self.transactions.insert_many(docs, None).await?;
        Ok(())
    }

    pub async fn get_available_to_budget(&self) -> mongodb::error::Result<f64> {
        let pipeline =
            vec![doc! {"$group": doc! {"_id": "sum", "spent": doc! {"$sum": "$amount"}}}];
        let mut results = self.transactions.aggregate(pipeline, None).await?;
        if let Some(sum) = results.try_next().await? {
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
