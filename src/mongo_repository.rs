use crate::repository::Repository;
use std::env;

use mongodb::{
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
}

fn connect() -> mongodb::error::Result<Client> {
    let connection_string = env::var("MONGO_STRING").unwrap();
    let mut client_options = ClientOptions::parse(connection_string)?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}
