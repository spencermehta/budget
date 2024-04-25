use chrono::NaiveDate;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

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
