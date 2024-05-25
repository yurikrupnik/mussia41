


pub fn routes() -> Router {
        .push(Router::with_path("hello").get(hello))
        .push(Router::with_path("hallo").get(hallo))
}