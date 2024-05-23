use ntex::web::{self, App, HttpRequest, HttpResponse};
use ntex::web::types::{Json, State};
use serde::{Deserialize, Serialize};
use tokio::task;
use std::sync::Arc;
use redis::AsyncCommands;
use shared::app_state::AppState;
// use redis::aio::ConnectionManager;

#[derive(Serialize, Deserialize)]
struct RequestData {
    message: String,
}

async fn handle_request(
    req: Json<RequestData>,
    pool: AppState,
) -> HttpResponse {
    let redis_pool = pool.redis;
    let mut conn = redis_pool.get().await.expect("Failed to get Redis connection from pool");
    let _: () = conn.publish("your-topic-name", &req.0.message).await.expect("Failed to publish message");

    HttpResponse::Ok().body("Message published")
}