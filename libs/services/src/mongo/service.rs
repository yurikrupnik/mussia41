use mongodb::bson::oid::{Error, ObjectId};
use mongodb::bson::{doc, from_document, to_document, Document};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use mongodb::{Collection, Database};
use ntex::web::HttpResponse;
use proc_macros::DbResource;
use serde::de::DeserializeOwned;
use serde::Serialize;
use shared::validation::validate_request_body;
use validator::Validate;
use anyhow::Result;
use serde::Deserialize;
fn create_query_id(id: &ObjectId) -> Document {
    doc! {"_id": id}
}

pub enum ResponseErrors {}

pub fn create_response<T>(response: Result<Option<Document>, Error>, id: ObjectId) -> HttpResponse
where
    T: Serialize + Sync + Send + Unpin + DeserializeOwned + 'static,
{
    match response {
        Ok(Some(payload)) => {
            let doc: T = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => HttpResponse::NotFound().json::<String>(&format!("No user found with id {id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

pub async fn create_item<T, U>(db: &Database, item: U) -> Result<Option<Document>>
where
    T: DbResource + Serialize + DeserializeOwned + Sync + Send + Unpin,
    U: Serialize + DeserializeOwned + Sync + Send + Unpin + Validate + 'static,
{
    let collection = db.collection(T::COLLECTION);
    let document = to_document(&item).expect("shit happened");
    let result = collection
        .insert_one(document, None)
        .await
        .expect("Error creating item");
    let new_id = result.inserted_id.as_object_id().expect("error 1");
    let response = collection.find_one(create_query_id(&new_id), None).await?;
    Ok(response)
    
    // match response {
    //     Ok(Some(payload)) => {
    //         let doc: T = from_document(payload).expect("error 2");
    //         HttpResponse::Created().json(&doc)
    //     }
    //     Ok(None) => {
    //         HttpResponse::NotFound().json::<String>(&format!("No user found with id {new_id}"))
    //     }
    //     Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    // }
}

pub async fn drop_collection<T: DbResource>(db: &Database) -> HttpResponse {
    let collection: Collection<Document> = db.collection(T::COLLECTION);
    collection.drop(None).await.unwrap();
    HttpResponse::Ok().body("successfully deleted!")
}

pub async fn delete_by_id<T>(db: &Database, id: &str) -> HttpResponse
where
    T: DbResource,
{
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = create_query_id(&obj_id);
    // let filter = doc! {"_id": &obj_id};
    let collection: Collection<T> = db.collection(T::COLLECTION);
    let result = collection
        .delete_one(filter, None)
        .await
        .expect("failed deleting");
    if result.deleted_count == 1 {
        HttpResponse::Ok().json(&"successfully deleted!")
    } else {
        HttpResponse::NotFound().json(&format!("item with specified ID {obj_id} not found!"))
    }
}
pub async fn get_by_id<T>(db: &Database, id: &str) -> HttpResponse
where
    T: DbResource + DeserializeOwned + Serialize + Sync + Send + Unpin,
{
    let collection: Collection<T> = db.collection(T::COLLECTION);
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
    let result = collection.find_one(filter, None).await;
    match result {
        Ok(Some(payload)) => HttpResponse::Ok().json(&payload),
        Ok(None) => HttpResponse::NotFound().json(&format!("No item found with id {obj_id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

pub async fn update_by_id<T, U>(db: &Database, body: U, id: &str) -> HttpResponse
where
    T: DbResource + DeserializeOwned + Serialize,
    U: DeserializeOwned + Serialize + Validate,
{
    if let Err(response) = validate_request_body(&body) {
        return response; // Returns early if validation fails
    }
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
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
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: T = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        None => HttpResponse::NotFound().json(&format!("not found item with ID {id}")),
    }
}

// Define a structure for the request body
#[derive(Deserialize)]
struct MyRequestBody {
    field: String,
}

// async fn validate_body<S>(
//     req: web::HttpRequest,
//     mut payload: web::Payload<S>,
// ) -> Result<web::HttpRequest, HttpResponse> {
//     let body = web::types::Json::<MyRequestBody>::from_request(&req, &mut payload).await;
//
//     match body {
//         Ok(_) => Ok(req),
//         Err(e) => Err(HttpResponse::BadRequest().body(e.to_string())),
//     }
// }

pub async fn update_by_id_v1<T, U>(db: &Database, body: U, id: &str) -> Result<Document>
where
    T: DbResource + DeserializeOwned + Serialize + Sync + Send + Unpin,
    U: DeserializeOwned + Serialize + Sync + Send + Unpin + Validate,
{
    body.validate()
        .map(|a| {})
        .map_err(|e| HttpResponse::BadRequest().body(e.to_string()));
    // if let Err(response) = validate_request_body(&body) {
    //     return response; // Returns early if validation fails
    // }
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! {"_id": obj_id};
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
        .await?
        .unwrap();
    Ok(result)
    // match result {
    //     Some(payload) => {
    //         let doc: T = from_document(payload).unwrap();
    //         HttpResponse::Created().json(&doc)
    //     }
    //     None => HttpResponse::NotFound().json(&format!("not found item with ID {id}")),
    // }
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
