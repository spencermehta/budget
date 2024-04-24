use mongodb::Collection;
use std::env;

use crate::{category::CategoryAssignment, transaction::Transaction};

pub struct Repository {
    pub db: mongodb::Database,
    pub transactions: Collection<Transaction>,
    pub categories: Collection<CategoryAssignment>,
}

use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

impl Repository {
    pub async fn new() -> Repository {
        let client = connect().await.unwrap();
        let db = client.database("transactions");
        let transactions = db.collection::<Transaction>("transactions");
        let categories = db.collection::<CategoryAssignment>("categories");

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
