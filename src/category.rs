use mongodb::bson::doc;

use crate::repository::Repository;

#[derive(Debug)]
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
}
