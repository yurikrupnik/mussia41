use axum::{
    routing::get,
    Router,
};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener_value = format!("{}:8080", Ipv4Addr::UNSPECIFIED);
    let listener = tokio::net::TcpListener::bind(listener_value).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}