mod api;
mod swagger;

// use std::sync::Arc;
use api::routes;
use general::socket_addrs::get_web_url;
use ntex::web::{self, get, HttpResponse};
use redis::AsyncCommands;
use shared::app_state::AppState;
use swagger::ApiDoc;
use services::{
    swagger::ntex::ntex_config,
    mongo::connector::connect as mongo_connect,
    redis::connector::connect as redis_connect
};
use bb8::{Pool,PooledConnection};
use bb8_redis::RedisConnectionManager;
use redis::aio::ConnectionLike;
use serde::{Serialize, Deserialize};
use services::redis::{publish::publish_message, subscribe::subscribe_to_channel};
use kube::{Client, api::{Api, ListParams}};
async fn list_pods(client: Client) {
    let pods: Api<k8s_openapi::api::core::v1::Pod> = Api::default_namespaced(client);
    let lp = ListParams::default().limit(10);
    match pods.list(&lp).await {
        Ok(pod_list) => {
            for p in pod_list {
                println!("Found Pod: {}", p.metadata.name.unwrap());
            }
        }
        Err(e) => println!("Error listing pods: {:?}", e),
    }
}

// async fn listen_to_topic(pool: Pool<RedisConnectionManager>) {
//     let mut conn = pool.get().await.expect("Failed to get Redis connection from pool");
//     let mut pubsub = conn.as_pubsub();
//     pubsub.subscribe("your-topic-name").await.expect("Failed to subscribe to topic");
//
//     loop {
//         let msg: String = pubsub.on_message().next().await.unwrap().get_payload().unwrap();
//         println!("Received message: {}", msg);
//     }
// }

async fn default() -> HttpResponse {
    HttpResponse::NotFound().finish()
}
#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    // let db = mongo_connect("aris").await;
    let db = mongo_connect("aris").await;
    let redis_pool = redis_connect().await;
    let ds = redis_pool.clone();
    let conn = ds.get().await.unwrap();

    let client = Client::try_default().await.expect("Failed to create client");
    list_pods(client.clone()).await;
    // conn.get_db().d
    // conn.se
    // let mut conn = &redis_pool.get().await.expect("Failed to get Redis connection from pool");
    // let mut pubsub = conn.publish("ds", None).await;
    // let sub_channel_name = "my_channel".to_string();
    // let sub_channel_name1 = "dam".to_string();
    // tokio::spawn(async move {
    //     subscribe_to_channel(&sub_channel_name).await.unwrap();
    //     subscribe_to_channel(&sub_channel_name1).await.unwrap();
    // });
    // tokio::spawn(async move {
    // });
    // // Small delay to ensure the subscriber is ready (not ideal in production)
    // tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //
    // // Publish a message
    // if let Err(e) = publish_message(&redis_pool, "my_channel", "Hello, world!").await {
    //     println!("Error publishing message: {}", e);
    // }

    let state = AppState::new(db, redis_pool);

    web::HttpServer::new(move || {
        let json_config = web::types::JsonConfig::default().limit(4096);
        web::App::new()
            // Register swagger endpoints
            .configure(ntex_config::<ApiDoc>)
            .wrap(web::middleware::Logger::default())
            .state(json_config)
            .state(state.clone())
            .configure(routes)
            .default_service(web::route().to(default))
    })
    .bind(get_web_url(false))?
    .workers(1)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    // use super::api::todo::controller::get_todo;
    use ntex::web::test;

    // #[ntex::test]
    // async fn test_index_ok() {
    //     let req = test::TestRequest::default()
    //         .header("content-type", "text/plain")
    //         .uri("/api/todo")
    //         .to_http_request();
    //     let resp = get_todo(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);
    // }

    // #[ntex::test]
    // async fn test_index_not_ok() {
    //     let req = test::TestRequest::default().to_http_request();
    //     let resp = get_todo(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
}
