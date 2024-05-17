// use redis::{aio::ConnectionLike, AsyncCommands, RedisResult};
// use futures::StreamExt;
// streams::StreamExt
use redis::{Client, ConnectionLike, RedisResult};

#[derive(Debug)]
struct Yr {}

pub async fn subscribe_to_channel(channel: &str) -> RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection()?;
    let mut pubsub = conn.as_pubsub();
    pubsub.subscribe(channel)?;

    loop {
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }

    // Ok(())
}
