use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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
pub struct SpentAggregate {
    pub spent: f64,
}
