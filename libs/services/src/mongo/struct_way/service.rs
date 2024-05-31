// see https://levelup.gitconnected.com/exploring-design-patterns-in-rust-21c83a0948dc
// took a lot from it
use general::get_mongo_uri;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
// use kube::Client;
use mongodb::bson::Document;
use mongodb::results::DeleteResult;
use mongodb::{Collection, Database};

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Singleton>> = Arc::new(Mutex::new(Singleton::new()));
}
struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { value: 0 }
    }

    fn get_instance() -> Arc<Mutex<Singleton>> {
        Arc::clone(&INSTANCE)
    }
}
struct MongoConnector {
    db: Database,
}

impl MongoConnector {
    async fn connect(database: &str) -> Self {
        let client = mongodb::Client::with_uri_str(get_mongo_uri())
            .await
            .expect("failed to connect");
        let db = client.database(database);
        Self { db }
    }
}

trait MongoService<T, C, U> {
    async fn create_item(&self, body: C) -> anyhow::Result<Option<Document>>;
    async fn drop_collection() -> anyhow::Result<()>;
    async fn delete_by_id() -> anyhow::Result<DeleteResult>;
    // async fn get_by_id(&self, id: &str) -> anyhow::Result<Option<T>> {
    //     println!("id {}", id);
    //     let collection: Collection<T> = db.collection("asd");
    //     let filter = crate::mongo::service::create_query_id(id);
    //     let result = collection.find_one(filter, None).await?;
    //     Ok(result)
    //     // let collection: Collection<T> = &self.db.collection(T::COLLECTION);
    //     // let filter = crate::mongo::service::create_query_id(id);
    //     // let result = collection.find_one(filter, None).await?;
    //     // Ok(result)
    // }
    async fn update_by_id() -> anyhow::Result<Option<Document>>;
}

pub struct Sd {
    db: Database,
}
