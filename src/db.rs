use mongodb::{Client, options::ClientOptions, Database};
use std::env;
use dotenv::dotenv;

pub async fn get_database() -> Database {
    dotenv().ok();
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");

    let client_options = ClientOptions::parse(&mongo_uri)
        .await
        .expect("Failed to parse MongoDB URI");

    let client = Client::with_options(client_options)
        .expect("Failed to connect to MongoDB");

    client.database("rust_auth")
}

