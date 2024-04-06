use std::env;

use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
};

use crate::transaction::Transaction;

pub fn connect() -> mongodb::error::Result<()> {
    let connection_string = env::var("MONGO_STRING").unwrap();

    let mut client_options = ClientOptions::parse(connection_string)?;

    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // Get a handle to the cluster
    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");

    for db_name in client.list_database_names(None, None)? {
        println!("{}", db_name);
    }

    let db = client.database("transactions");

    let collection = db.collection::<Transaction>("transactions");

    let docs = vec![Transaction {
        party: "sainsbury's".to_string(),
        category: "Groceries".to_string(),
        amount: 100.0,
    }];

    collection.insert_many(docs, None)?;

    Ok(())
}
