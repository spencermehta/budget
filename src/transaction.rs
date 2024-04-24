use crate::repository::Repository;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use futures_util::TryStreamExt;
use mongodb::bson::doc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub budget_id: String,
    pub date: NaiveDate,
    pub party: String,
    pub category: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SpentAggregate {
    spent: f64,
}

impl Repository {
    pub async fn find_transactions(
        &self,
        budget_id: String,
    ) -> mongodb::error::Result<Vec<Transaction>> {
        let mut cursor = self
            .transactions
            .find(doc! { "budget_id": budget_id }, None)
            .await?;
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
