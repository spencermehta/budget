use bson;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

use crate::repository::Repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub spent: f64,
}

impl Repository {
    pub fn list_categories(&self) -> mongodb::error::Result<Vec<String>> {
        let categories = self.transactions.distinct("category", doc! {}, None)?;
        let mut category_names = Vec::new();
        for category in categories {
            category_names.push(category.as_str().unwrap().to_string());
        }
        Ok(category_names)
    }

    pub fn category_spends(&self) -> mongodb::error::Result<Vec<Category>> {
        let pipeline = vec![
            doc! {"$group": doc! {"_id": "$category", "spent": doc! {"$sum": "$amount"}}},
            doc! {
                "$project": doc! {
                    "name": "$_id",
                    "spent": "$spent"
                }

            },
        ];
        let results = self.transactions.aggregate(pipeline, None)?;
        let mut categories = Vec::new();
        for res in results {
            match res {
                Ok(category) => {
                    let doc = bson::from_document::<Category>(category);
                    categories.push(doc.unwrap());
                }
                Err(e) => println!("{}", e),
            }
        }

        Ok(categories)
    }
}
