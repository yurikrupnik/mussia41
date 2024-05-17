use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_document, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{Collection, Database};
use proc_macros::DbResource;
use serde::de::DeserializeOwned;
use serde::Serialize;
use anyhow::Result;
use mongodb::results::DeleteResult;

fn create_query_id(id: &str) -> Document {
    let obj_id = ObjectId::parse_str(id).unwrap();
    doc! {"_id": obj_id}
}

pub enum ResponseErrors {}

pub async fn create_item<T, U>(db: &Database, item: U) -> Result<Option<Document>>
where
    T: DbResource + Serialize + DeserializeOwned + Sync + Send + Unpin,
    U: Serialize + DeserializeOwned + Sync + Send + Unpin + 'static,
{
    let collection = db.collection(T::COLLECTION);
    let document = to_document(&item).expect("shit happened");
    let result = collection
        .insert_one(document, None)
        .await?;
    let sd = result.inserted_id.as_str().expect("error 1");
    let response = collection.find_one(create_query_id(sd), None).await?;
    Ok(response)
}

pub async fn drop_collection<T: DbResource>(db: &Database) -> Result<()> {
    let collection: Collection<Document> = db.collection(T::COLLECTION);
    collection.drop(None).await?;
    Ok(())
}

pub async fn delete_by_id<T>(db: &Database, id: &str) -> Result<DeleteResult>
where
    T: DbResource,
{
    let filter = create_query_id(id);
    let collection: Collection<T> = db.collection(T::COLLECTION);
    let result = collection
        .delete_one(filter, None)
        .await?;
    Ok(result)
}
pub async fn get_by_id<T>(db: &Database, id: &str) -> Result<Option<T>>
where
    T: DbResource + DeserializeOwned + Serialize + Sync + Send + Unpin,
{
    let collection: Collection<T> = db.collection(T::COLLECTION);
    let filter = create_query_id(id);
    let result = collection.find_one(filter, None).await?;
    Ok(result)
}

pub async fn update_by_id<T, U>(db: &Database, body: U, id: &str) -> Result<Option<Document>>
where
    T: DbResource + DeserializeOwned + Serialize,
    U: DeserializeOwned + Serialize,
{
    let filter = create_query_id(id);
    let collection: Collection<Document> = db.collection(T::COLLECTION);
    let serialized_item = to_document(&body).expect("Error serializing item");
    let new_doc = doc! {
        "$set": serialized_item
    };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();
    let result = collection
        .find_one_and_update(filter, new_doc, options)
        .await?;
    Ok(result)
}


#[cfg(test)]
mod tests {
    use serde::Deserialize;
    // use utoipa::IntoParams;
    use super::*;

    struct Test1 {
        name: String,
    }
    struct CreateTest1 {
        name: String,
    }
    struct UpdateTest1 {
        name: String,
    }

    #[derive(Clone, Deserialize, Serialize, Debug)]
    #[serde(deny_unknown_fields)]
    pub struct TodoQueryParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        completed: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        text: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub total: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub skip: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub projection: Option<String>,
    }
    #[test]
    fn generic_routes_basic() {
        // let db =
        // let s = delete_by_id::<Test1>(db: &Database, id: &str);
        // let s = generic_routes::<Test1, CreateTest1, UpdateTest1,TodoQueryParams>();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
