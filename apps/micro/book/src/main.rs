use futures_util::future::join_all;
use ntex::rt;
// use redis::AsyncCommands;
use bb8::Pool;
use general::{get_mongo_uri, get_redis_uri};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tokio::select;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    age: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct OtherData {
    info: String,
}

use bb8_redis::{
    bb8,
    redis::{cmd, AsyncCommands},
    RedisConnectionManager,
};
use futures_util::StreamExt;
use redis::Msg;

async fn maadsin() {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = bb8::Pool::builder().build(manager).await.unwrap();

    let mut handles = vec![];

    for _i in 0..10 {
        let pool = pool.clone();

        handles.push(tokio::spawn(async move {
            let mut conn = pool.get().await.unwrap();

            let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();

            assert_eq!("PONG", reply);
        }));
    }

    join_all(handles).await;
}
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let manager = RedisConnectionManager::new(get_redis_uri()).unwrap();
    // let redis_pool = Pool::builder().build(manager).await.unwrap();
    let client = redis::Client::open(get_redis_uri()).unwrap();
    let mut con = client.get_connection().unwrap();

    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("person_topic").unwrap();

    loop {
        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
    // let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    // let mut con = client.get_async_connection().await.unwrap();
    // let mut pubsub = con.as_pubsub();
    // pubsub.subscribe("person_topic").await.unwrap();
    // pubsub.subscribe("other_topic").await.unwrap();
    //
    // loop {
    //     select! {
    //         msg = pubsub.on_message() => {
    //             let channel: String = msg.get_channel_name().to_string();
    //             let payload: String = msg.get_payload().unwrap();
    //
    //             match channel.as_str() {
    //                 "person_topic" => {
    //                     let person: Person = serde_json::from_str(&payload).unwrap();
    //                     println!("Received on person_topic: {:?}", person);
    //                 },
    //                 "other_topic" => {
    //                     let other_data: OtherData = serde_json::from_str(&payload).unwrap();
    //                     println!("Received on other_topic: {:?}", other_data);
    //                 },
    //                 _ => {
    //                     println!("Received on unknown topic: {}", channel);
    //                 }
    //             }
    //         }
    //     }
    // }
}
