use crate::repository::Repository;
use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
    sync::Collection,
};

use crate::transaction::Transaction;

impl Repository {
    pub fn new() -> Repository {
        let collection = connect().unwrap();
        Repository { collection }
    }

    pub fn find_transaction(&self) -> mongodb::error::Result<()> {
        let cursor = self.collection.find(doc! {}, None)?;
        for result in cursor {
            println!("party: {}", result?.party);
        }

        Ok(())
    }
}

pub fn connect() -> mongodb::error::Result<Collection<Transaction>> {
    let connection_string = env::var("MONGO_STRING").unwrap();
    let mut client_options = ClientOptions::parse(connection_string)?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;

    let db = client.database("transactions");
    let collection = db.collection::<Transaction>("transactions");

    Ok(collection)
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
