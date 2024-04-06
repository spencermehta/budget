use mongodb::sync::Collection;

use crate::transaction::Transaction;

pub struct Repository {
    pub db: mongodb::sync::Database,
    pub transactions: Collection<Transaction>,
}
