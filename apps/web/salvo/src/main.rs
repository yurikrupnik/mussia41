use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

use general::socket_addrs::get_web_url_v1;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let listener = get_web_url_v1();
    let acceptor = TcpListener::new(listener).bind().await;
    Server::new(acceptor).serve(router).await;
}
