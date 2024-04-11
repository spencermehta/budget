use crate::input;
use bson;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::repository::Repository;
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    transaction_category: TransactionCategory,
    budget_category: BudgetCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCategory {
    pub name: String,
    pub spent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetCategory {
    pub name: String,
    pub budget: f64,
}

impl Repository {
    pub async fn list_categories(&self) -> mongodb::error::Result<Vec<String>> {
        let categories = self
            .transactions
            .distinct("category", doc! {}, None)
            .await?;
        let mut category_names = Vec::new();
        for category in categories {
            category_names.push(category.as_str().unwrap().to_string());
        }
        Ok(category_names)
    }

    pub async fn category_spends(&self) -> mongodb::error::Result<Vec<Category>> {
        let pipeline = vec![
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
                .get_budget_for_category(&transaction_category.name)
                .await?;
            categories.push(Category {
                transaction_category,
                budget_category,
            });
        }

        Ok(categories)
    }

    pub async fn set_budget_for_category(
        &self,
        category: BudgetCategory,
    ) -> mongodb::error::Result<()> {
        let docs = vec![category];
        self.categories.insert_many(docs, None).await?;
        Ok(())
    }

    pub async fn get_budget_for_category(
        &self,
        category_name: &String,
    ) -> mongodb::error::Result<BudgetCategory> {
        let cursor = self
            .categories
            .find_one(doc! { "name": category_name }, None)
            .await?;

        match cursor {
            Some(category) => Ok(category),
            None => Ok(BudgetCategory {
                name: category_name.to_string(),
                budget: 0.0,
            }),
        }
    }
}

pub fn create_category() -> BudgetCategory {
    println!("Category:");
    let name = input::get_input();
    println!("Budget:");
    let budget = input::get_float_input();

    BudgetCategory { name, budget }
}
