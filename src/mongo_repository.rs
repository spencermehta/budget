use crate::{category::BudgetCategory, repository::Repository};
use std::env;

use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use crate::transaction::Transaction;

impl Repository {
    pub async fn new() -> Repository {
        let client = connect().await.unwrap();
        let db = client.database("transactions");
        let transactions = db.collection::<Transaction>("transactions");
        let categories = db.collection::<BudgetCategory>("categories");

        Repository {
            db,
            transactions,
            categories,
        }
    }
}

async fn connect() -> mongodb::error::Result<Client> {
    let connection_string = env::var("MONGO_STRING").unwrap();
    let mut client_options = ClientOptions::parse(connection_string).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}
