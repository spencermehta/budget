use std::env;

use crate::category::{Category, TransactionCategory};
use crate::transaction::SpentAggregate;
use crate::{category::CategoryAssignment, repository::Repository, transaction::Transaction};
use bson::doc;
use chrono::prelude::*;
use chrono::NaiveDate;
use futures_util::TryStreamExt;
use mongodb::options::ReplaceOptions;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection,
};

pub struct MongoRepository {
    pub db: mongodb::Database,
    pub transactions: Collection<Transaction>,
    pub categories: Collection<CategoryAssignment>,
}

impl MongoRepository {
    pub async fn new() -> MongoRepository {
        let client = connect().await.unwrap();
        let db = client.database("transactions");
        let transactions = db.collection::<Transaction>("transactions");
        let categories = db.collection::<CategoryAssignment>("categories");

        MongoRepository {
            db,
            transactions,
            categories,
        }
    }
}

impl Repository for MongoRepository {
    async fn find_transactions(&self, budget_id: String) -> anyhow::Result<Vec<Transaction>> {
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

    async fn insert_transaction(&self, transaction: Transaction) -> anyhow::Result<()> {
        let docs = vec![transaction];
        self.transactions.insert_many(docs, None).await?;
        Ok(())
    }

    async fn list_categories(&self, budget_id: String) -> anyhow::Result<Vec<String>> {
        let categories = self
            .transactions
            .distinct("category", doc! { "budget_id": budget_id }, None)
            .await?;
        let mut category_names = Vec::new();
        for category in categories {
            category_names.push(category.as_str().unwrap().to_string());
        }
        Ok(category_names)
    }

    async fn category_spends(
        &self,
        budget_id: String,
        date: NaiveDate,
    ) -> anyhow::Result<Vec<Category>> {
        let date_filter_lower = format!("{}-{:0>2}", date.year(), date.month());
        let date_filter_upper = format!("{}-{:0>2}", date.year(), date.month() + 1);

        let pipeline = vec![
            doc! {"$match": doc! {
                "budget_id": &budget_id,
                "date": {
                    "$lt": date_filter_upper,
                    "$gte": date_filter_lower,
                }
            }},
            doc! {"$group": doc! {"_id": "$category", "spent": doc! {"$sum": "$amount"}}},
            doc! {
                "$project": doc! {
                    "name": "$_id",
                    "spent": "$spent"
                }

            },
        ];
        let mut results = self.transactions.aggregate(pipeline, None).await?;
        let mut categories = Vec::new();

        while let Some(category) = results.try_next().await? {
            let doc = bson::from_document::<TransactionCategory>(category);
            let transaction_category = doc.unwrap();
            let budget_category = self
                .get_category_assignment(&budget_id, &transaction_category.name, date)
                .await?;
            categories.push(Category {
                name: transaction_category.name,
                spent: transaction_category.spent,
                assigned: budget_category.assigned,
            });
        }

        Ok(categories)
    }

    async fn assign_to_category(&self, category: CategoryAssignment) -> anyhow::Result<()> {
        let date_filter_lower = format!("{}-{:0>2}", category.date.year(), category.date.month());
        let date_filter_upper =
            format!("{}-{:0>2}", category.date.year(), category.date.month() + 1);

        let filter = doc! { "name": &category.name, "date": {
            "$lt": date_filter_upper,
            "$gte": date_filter_lower,
        } };

        let options = ReplaceOptions::builder().upsert(true).build();

        self.categories
            .replace_one(filter, category, options)
            .await?;
        Ok(())
    }

    async fn get_category_assignment(
        &self,
        budget_id: &String,
        category_name: &String,
        date: NaiveDate,
    ) -> anyhow::Result<CategoryAssignment> {
        let date_filter_lower = format!("{}-{:0>2}", date.year(), date.month());
        let date_filter_upper = format!("{}-{:0>2}", date.year(), date.month() + 1);

        let cursor = self
            .categories
            .find_one(
                doc! { "budget_id": budget_id, "name": category_name, "date": {
                    "$lt": date_filter_upper,
                    "$gt": date_filter_lower,
                } },
                None,
            )
            .await?;

        match cursor {
            Some(category) => Ok(category),
            None => Ok(CategoryAssignment {
                budget_id: budget_id.to_string(),
                name: category_name.to_string(),
                assigned: 0.0,
                date,
            }),
        }
    }
}

impl MongoRepository {
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

async fn connect() -> mongodb::error::Result<Client> {
    let connection_string = env::var("MONGO_STRING").unwrap();
    let mut client_options = ClientOptions::parse(connection_string).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}
