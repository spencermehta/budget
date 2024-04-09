use mongodb::sync::Collection;

use crate::{category::BudgetCategory, transaction::Transaction};

pub struct Repository {
    pub db: mongodb::sync::Database,
    pub transactions: Collection<Transaction>,
    pub categories: Collection<BudgetCategory>,
}
