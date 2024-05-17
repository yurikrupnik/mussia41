use axum::{routing::get, Router};
use general::socket_addrs::get_web_url_v1;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // let port = get_port();
    // run our app with hyper, listening globally on port
    // let listener_value = format!("{}:{}", Ipv4Addr::UNSPECIFIED, port);
    let listener_value = get_web_url_v1();
    let listener = tokio::net::TcpListener::bind(listener_value).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
