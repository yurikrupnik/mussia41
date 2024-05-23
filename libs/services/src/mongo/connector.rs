use mongodb::{Client, Database};
use general::{get_mongo_uri};

pub async fn connect(database: &str) -> Database {
    let client = Client::with_uri_str(get_mongo_uri())
        .await
        .expect("failed to connect");
    client.database(database)
} 