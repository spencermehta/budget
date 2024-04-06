use crate::repository::Repository;
use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
};

use crate::transaction::Transaction;

impl Repository {
    pub fn new() -> Repository {
        let client = connect().unwrap();
        let db = client.database("transactions");
        let transactions = db.collection::<Transaction>("transactions");
        Repository { db, transactions }
    }

    pub fn find_transaction(&self) -> mongodb::error::Result<Vec<Transaction>> {
        let cursor = self.transactions.find(doc! {}, None)?;
        let mut txns = Vec::new();
        for txn in cursor {
            txns.push(txn.unwrap())
        }

        Ok(txns)
    }
}

fn connect() -> mongodb::error::Result<Client> {
    let connection_string = env::var("MONGO_STRING").unwrap();
    let mut client_options = ClientOptions::parse(connection_string)?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub fn insert() {
    // let collection = db.collection::<Transaction>("transactions");
    let docs = vec![Transaction {
        party: "sainsbury's".to_string(),
        category: "Groceries".to_string(),
        amount: 100.0,
    }];

    // collection.insert_many(docs, None)?;
}
