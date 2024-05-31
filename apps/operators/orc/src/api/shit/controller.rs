use super::model::{NewShit, Shit, ShitItemQuery, ShitListQuery, UpdateShit};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, from_document};
use mongodb::Collection;
use ntex::web::types::{Json, Path, Query, State};
use ntex::web::HttpResponse;
use proc_macros::DbResource;
use serde::{Deserialize, Serialize};
use services::mongo::filter_and_options::construct_find_options_and_filter;
use services::mongo::service::{
    create_item, delete_by_id, drop_collection, get_by_id, update_by_id,
};
// use tokio::
use bb8_redis::RedisConnectionManager;
use redis::{self, aio::PubSub as AioPubsub, cmd, PubSub, PubSubCommands};
use shared::app_state::AppState;
use shared::validation::validate_request_body;
// use futures::prelude::*;
// use redis::{AsyncCommands};

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    message: String,
}

// fn sda(mut con:  PooledConnection<RedisConnectionManager>) {
//     let s =  con.publish("create_book", new_shit_str).await;
//     s
// }
pub async fn handle_request(body: Json<NewShit>, app_state: State<AppState>) -> HttpResponse {
    let new_shit = body.into_inner();
    let pool = app_state.redis.clone();
    let new_shit_str = serde_json::to_string(&new_shit).unwrap();
    tokio::spawn(async move {
        let mut conn = pool.get().await.unwrap();
        println!("hello!!!!");
        // let count : i32 = conn.get("my_counter")?;
        // redis::cmd("set").arg("aor").arg(23).query_async(&mut *conn).await.unwrap();
        let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();
        // let rep  = cmd("SET").arg("my_shit").arg(42).query_async(&mut *conn).await.unwrap();
        // let rep  = cmd("SET").arg("my_key").arg(42).query(&mut *conn).await.unwrap();
        assert_eq!("PONG", reply);
    });
    // let result = pool.get().await.and_then(|mut con| {
    //     // let s =  con.publish("create_book", new_shit_str);
    //     // con.p
    //     //
    //     Ok(())
    // });
    //
    // match result {
    //     Ok(_) => HttpResponse::build(StatusCode::CREATED).finish(),
    //     Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).finish(),
    // }
    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut con = client.get_connection().unwrap();
    // // con.subscribe() // works
    // let event = serde_json::to_string(&*body).unwrap();
    // // let _: () = con.publish("book_events", event).unwrap();

    // let _: () = con
    // HttpResponse::build(StatusCode::CREATED).finish()
    // let redis_pool = &app_state.redis;
    // let mut con = redis_pool.get().await.expect("Failed to connect to Redis");
    // con.se
    // let _: () = con.set("latest_request", &payload.data).expect("Failed to set value in Redis");
    // con.se
    //     let mut conn = redis_pool.get_owned().await.expect("Failed to get Redis connection from pool");
    // let mut conn = redis_pool.get().await.expect("Failed to get Redis connection from pool");
    // conn
    // let _ : () = conn.set("my_key", RequestData{message: "dsa".to_string()}).await.expect("dsa");
    // let _: () = conn.publish("dam", &body.0.message).await.expect("Failed to publish message");

    HttpResponse::Ok().body("Message published")
}

/// Get a Shit by id
#[utoipa::path(
    get,
    path = "/api/shit/{id}",
    tag = Shit::TAG,
    responses(
        (status = 200, description = "Shit found", body = Shit),
        (status = 404, description = "Shit not found", body = HttpError),
    ),
)]
pub async fn get_shit(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    // let query = query.into_inner();
    // let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let result = get_by_id::<Shit>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => HttpResponse::Ok().json(&payload),
        Ok(None) => HttpResponse::NotFound().json(&format!("No item found with id {item_id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// List all shits
#[utoipa::path(
    get,
    path = "/api/shit",
    tag = Shit::TAG,
    params(ShitListQuery),
    responses(
        (status = 200, description = "List of Shit", body = [Shit]),
    ),
)]
pub async fn get_shits(app_state: State<AppState>, query: Query<ShitListQuery>) -> HttpResponse {
    let query = query.into_inner();
    let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let db = &app_state.db;
    let collection: Collection<Shit> = db.collection(Shit::COLLECTION);
    let mut cursor = collection
        .find(filter, options)
        .await
        .expect("failed fetching");
    let mut payload: Vec<Shit> = Vec::new();
    while let Some(item) = cursor
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        payload.push(item);
    }
    HttpResponse::Ok().json(&payload)
}

/// Create a new Shit
#[utoipa::path(
    post,
    path = "/api/shit",
    tag = Shit::TAG,
    request_body = NewShit,
    responses(
    (status = 201, description = "Shit created", body = Shit),
    ),
)]
pub async fn create_shit(body: Json<NewShit>, app_state: State<AppState>) -> HttpResponse {
    let body = body.into_inner();
    let db = &app_state.db;
    if let Err(response) = validate_request_body(&body) {
        return HttpResponse::BadRequest().json(&response);
    }
    let response = create_item::<Shit, NewShit>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Shit = from_document(payload).expect("error 5");
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => HttpResponse::NotFound().json::<String>(&format!("No user found with id")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// Delete `Shit` by ID
#[utoipa::path(
  delete,
  path = "/api/shit/{id}",
  tag = Shit::TAG,
  responses(
    (status = 200, description = "Shit deleted", body = String),
    (status = 404, description = "Shit not found", body = HttpError),
  ),
)]
pub async fn delete_shit(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = delete_by_id::<Shit>(db, &item_id).await;
    match result {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json(&"successfully deleted!")
            } else {
                HttpResponse::NotFound()
                    .json(&format!("item with specified ID {item_id} not found!"))
            }
        }
        Err(_) => {
            HttpResponse::NotFound().json(&format!("item with specified ID {item_id} not found!"))
        }
    }
}

/// Delete all `Shit`
#[utoipa::path(
    delete,
    path = "/api/shit",
    tag = Shit::TAG,
    responses(
        (status = 200, description = "Shit deleted", body = String),
    ),
)]
pub async fn drop_shits(app_state: State<AppState>) -> HttpResponse {
    let db = &app_state.db;
    drop_collection::<Shit>(db).await.expect("das");
    HttpResponse::Ok().body("successfully deleted!")
}

/// Update a `Shit` by id and `UpdateShit` struct.
#[utoipa::path(
    put,
    path = "/api/shit/{id}",
    tag = Shit::TAG,
    request_body = UpdateShit,
    responses(
    (status = 200, description = "Shit updated", body = Shit),
    (status = 404, description = "Shit not found", body = HttpError),
    ),
)]
pub async fn update_shit(
    id: Path<String>,
    app_state: State<AppState>,
    body: Json<UpdateShit>,
) -> HttpResponse {
    let body = body.into_inner();
    if let Err(response) = validate_request_body(&body) {
        return HttpResponse::BadRequest().json(&response);
    }
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = update_by_id::<Shit, UpdateShit>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Shit = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        None => HttpResponse::NotFound().json(&format!("not found item with ID {item_id}")),
    }
}
