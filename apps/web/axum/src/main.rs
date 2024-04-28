use axum::{
    routing::get,
    Router,
};
use std::net::Ipv4Addr;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let sf = Ipv4Addr::UNSPECIFIED;
    // run our app with hyper, listening globally on port 3000
    let s = format!("{sf}:3000");
    let listener = tokio::net::TcpListener::bind(s).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}