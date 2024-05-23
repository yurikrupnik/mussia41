extern crate redis;
// use redis::Commands;
use redis::{Client, PubSubCommands};
use general::get_redis_uri;
use redis::{Connection, RedisResult, aio::AsyncStream};
use futures::prelude::*;
// use redis::AsyncCommands;
// use redis::Commands;
// use redis::aio::ConnectionLike;
// use tokio::net::TcpListener;
// use tokio::task;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
fn do_something(con: &mut Connection) -> RedisResult<()> {
    let _ : () = redis::cmd("SET").arg("my_key").arg(42).query(con)?;
    // let _ : () = con.set("my_key", 42)?;
    // This will result in a server error: "unknown command `MEMORY USAGE`"
    // because "USAGE" is technically a sub-command of "MEMORY".
    // redis::cmd("MEMORY USAGE").arg("my_key").query(con)?;
    // let s = con.subscribe("dsa").unwrap();
    // However, this will work as you'd expect
    // redis::cmd("MEMORY").arg("USAGE").arg("my_key").query(con).expect("TODO: panic message");
    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::open(get_redis_uri())?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();
    pubsub.subscribe("channel_1")?;
    // pubsub.subscribe("channel_2")?;
    loop {
        let msg = pubsub.get_message()?;
        let payload : String = msg.get_payload()?;
        println!("channel '{}': {}", msg.get_channel_name(), payload);
    }
    // let conn = client.create_multiplexed_tokio_connection().await.unwrap();
    // let con = client.get_multiplexed_tokio_connection().await?.clone();
    // let s = do_something(conn);
    // client.get_async_pubsub().await.unwrap().subscribe("csad").await?;
    // con.se
    // con.
    // con.sse
    Ok(())
    //     let listener = TcpListener::bind("0.0.0.0:8081").await?;
    //
    // loop {
    //     let (mut socket, _) = listener.accept().await?;
    //
    //     tokio::spawn(async move {
    //         let mut buf = [0; 1024];
    //
    //         // In a loop, read data from the socket and write the data back.
    //         loop {
    //             let n = match socket.read(&mut buf).await {
    //                 // socket closed
    //                 Ok(n) if n == 0 => return,
    //                 Ok(n) => n,
    //                 Err(e) => {
    //                     eprintln!("failed to read from socket; err = {:?}", e);
    //                     return;
    //                 }
    //             };
    //
    //             // Write the data back
    //             if let Err(e) = socket.write_all(&buf[0..n]).await {
    //                 eprintln!("failed to write to socket; err = {:?}", e);
    //                 return;
    //             }
    //         }
    //     });
    // }
}