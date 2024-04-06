use mongodb::sync::Collection;

use crate::transaction::Transaction;

pub struct Repository {
    pub collection: Collection<Transaction>,
}
