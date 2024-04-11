use mongodb::Collection;

use crate::{category::BudgetCategory, transaction::Transaction};

pub struct Repository {
    pub db: mongodb::Database,
    pub transactions: Collection<Transaction>,
    pub categories: Collection<BudgetCategory>,
}
