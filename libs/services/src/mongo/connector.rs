use general::get_mongo_uri;
use mongodb::{Client, Database};

pub async fn connect(database: &str) -> Database {
    let client = Client::with_uri_str(get_mongo_uri())
        .await
        .expect("failed to connect");
    client.database(database)
}

#[cfg(test)]
mod tests {
    // use crate::mongo::connector::connect;
    use super::*;

    #[tokio::test]
    async fn test_connect() {
        let db_name = "test_db";
        let db = connect(db_name).await;

        assert_eq!(db.name(), db_name);
    }
}
