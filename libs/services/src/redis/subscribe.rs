// use redis::{aio::ConnectionLike, AsyncCommands, RedisResult};
// use futures::StreamExt;
// streams::StreamExt
use general::get_redis_uri;
use redis::{Client, RedisResult};

pub async fn subscribe_to_channel(channel: &str) -> RedisResult<()> {
    let client = Client::open(get_redis_uri()).unwrap();
    let mut conn = client.get_connection()?;
    // conn.publish("ADS")
    let mut pubsub = conn.as_pubsub();
    pubsub.subscribe(channel)?;

    loop {
        let msg = pubsub.get_message()?;
        let payload: String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }

    // Ok(())
}
