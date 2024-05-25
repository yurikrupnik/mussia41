mod api;

use general::socket_addrs::get_web_url_v1;
// use salvo::catcher::Catcher;
// use salvo::http::header::HeaderValue;
use salvo::logging::Logger;
use salvo::prelude::*;
// #[endpoint(status_codes(200, 409), tags("Default"))]
// async fn hallo(name: QueryParam<String, false>) -> String {
//     format!("Hallo, {}!", name.as_deref().unwrap_or("World"))
// }

fn new_route() -> Router {
    Router::with_path("/api").hoop(Logger::new())
    // .push(user_router())
    // .push(author_router())
    // .push(Router::with_path("hello").get(hello))
    // .push(Router::with_path("hallo").get(hallo))
}

#[handler]
async fn handle404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render("Custom 404 Error Page");
        ctrl.skip_rest();
    }
}

fn create_service() -> Service {
    // Service::new().push(Router::with_path("hello").get(hello))
    let router = Router::new().push(Router::with_path("hello"));
    Service::new(router)
    // .catcher(Catcher::default().hoop(handle404))
    // .push(Router::with_path("hallo").get(hallo))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .hoop(CachingHeaders::new())
        .hoop(Compression::new().min_length(0))
        .push(new_route());
    // .push(Router::with_path("hello").get(hello));

    let doc = OpenApi::new("Salvo Api", "0.0.1").merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"));

    // let router = Router::new().get(hello).hoop();
    let listener = get_web_url_v1();
    let acceptor = TcpListener::new(listener).bind().await;
    Server::new(acceptor).serve(router).await;
}
