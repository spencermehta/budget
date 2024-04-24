use bson;
use chrono::prelude::*;
use chrono::NaiveDate;
use mongodb::{bson::doc, options::ReplaceOptions};
use serde::{Deserialize, Serialize};

use crate::repository::Repository;
use futures_util::TryStreamExt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub spent: f64,
    pub assigned: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCategory {
    pub name: String,
    pub spent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryAssignment {
    pub budget_id: String,
    pub name: String,
    pub assigned: f64,
    pub date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryExpenditureInput {
    pub budget_id: String,
    pub date: NaiveDate,
}

impl Repository {
    pub async fn list_categories(&self, budget_id: String) -> mongodb::error::Result<Vec<String>> {
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

    pub async fn category_spends(
        &self,
        budget_id: String,
        date: NaiveDate,
    ) -> mongodb::error::Result<Vec<Category>> {
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
                .get_budget_for_category(&budget_id, &transaction_category.name, date)
                .await?;
            categories.push(Category {
                name: transaction_category.name,
                spent: transaction_category.spent,
                assigned: budget_category.assigned,
            });
        }

        Ok(categories)
    }

    pub async fn set_budget_for_category(
        &self,
        category: CategoryAssignment,
    ) -> mongodb::error::Result<()> {
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

    pub async fn get_budget_for_category(
        &self,
        budget_id: &String,
        category_name: &String,
        date: NaiveDate,
    ) -> mongodb::error::Result<CategoryAssignment> {
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
