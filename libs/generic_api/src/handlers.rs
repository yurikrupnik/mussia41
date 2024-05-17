use futures::TryStreamExt;
use mongodb::bson::{from_document, Document};
use mongodb::Collection;
use ntex::web::types::{Json, Path, Query, State};
use ntex::web::{HttpResponse, Responder};
use proc_macros::DbResource;
use serde::de::DeserializeOwned;
use serde::Serialize;
use services::mongo::filter_and_options::construct_find_options_and_filter;
use services::mongo::query_param_processing::QueryParamProcessing;
use services::mongo::service::{
    create_item, delete_by_id, drop_collection, get_by_id, update_by_id,
};
use shared::app_state::AppState;
use validator::Validate;

/// Create new generic CRUD (delete) function.
/// Uses generic `drop_collection::<T>(db).await`
/// Resets body and db on webserver state
pub async fn drop<T: DbResource>(app_state: State<AppState>) -> impl Responder {
    let db = &app_state.db;
    drop_collection::<T>(db).await
}

/// Create new generic CRUD (create) function.
/// Uses generic `create_item::<T,U>(db, body).await`
/// Extracts body and db from webserver state
pub async fn create<T, C>(body: Json<C>, app_state: State<AppState>) -> HttpResponse
where
    T: DbResource + Serialize + DeserializeOwned + Sync + Send + Unpin,
    C: Serialize + DeserializeOwned + Sync + Send + Unpin + Validate + 'static,
{
    let body = body.into_inner();
    let db = &app_state.db;
    let response = create_item::<T, C>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: T = from_document(payload).expect("error 5");
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => {
            HttpResponse::NotFound().json::<String>(&format!("No user could be created from the data"))
        }
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// Delete `Project` by ID
pub async fn delete<T>(app_state: State<AppState>, id: Path<String>) -> impl Responder
where
    T: DbResource,
{
    let item_id = id.into_inner();
    let db = &app_state.db;
    delete_by_id::<T>(db, &item_id).await
}

/// Get `Project` by ID
pub async fn get_item<T, Q>(app_state: State<AppState>, id: Path<String>) -> impl Responder
where
    T: DbResource + DeserializeOwned + Serialize + Sync + Send + Unpin,
    // Q:
{
    let item_id = id.into_inner();
    let db = &app_state.db;
    get_by_id::<T>(db, &item_id).await
}

/// Update `Project` by `UpdateProject` struct
pub async fn update<T, U>(
    body: Json<U>,
    id: Path<String>,
    app_state: State<AppState>,
) -> impl Responder
where
    T: DbResource + Serialize + DeserializeOwned,
    U: Validate + Serialize + DeserializeOwned,
{
    let body = body.into_inner();
    let item_id = id.into_inner();
    let db = &app_state.db;
    update_by_id::<T, U>(db, body, &item_id).await
}

/// Get list by `T` and `Q`
pub async fn get_list<T, Q>(app_state: State<AppState>, query: Query<Q>) -> impl Responder
where
    T: DbResource + Serialize + DeserializeOwned,
    Q: Serialize + DeserializeOwned + QueryParamProcessing + Clone,
{
    let query = query.into_inner();
    // println!("{query}");
    let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let db = &app_state.db;
    let collection: Collection<Document> = db.collection(T::COLLECTION);
    let mut cursor = collection
        .find(filter, options)
        .await
        .expect("failed fetching");
    let mut payload: Vec<T> = Vec::new();
    while let Some(item) = cursor
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        let doc: T = from_document(item).unwrap();
        payload.push(doc);
    }

    HttpResponse::Ok().json(&payload)
}
