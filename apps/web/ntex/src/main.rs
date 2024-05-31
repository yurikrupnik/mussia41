use general::socket_addrs::get_web_url;
use ntex::web::{self, HttpResponse, Responder};

#[web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    web::HttpServer::new(move || {
        let json_config = web::types::JsonConfig::default().limit(4096);
        web::App::new()
            // .service(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            // .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            // .service(Redoc::with_url("/redoc", ApiDoc::openapi.clone()))
            .wrap(web::middleware::Logger::default())
            .state(json_config)
            // .state(redis_pool.clone())
            // .state(client.clone())
            // .state(state.clone())
            .service(hello)
        // .configure(user_routes)
        // .configure(project_routes)
        // .configure(generic_project_routes::<Project, NewProject>)
        // .configure(user_routes_mongo)
        // .default_service()
    })
    .bind(get_web_url(false))?
    .workers(1)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use ntex::web;
    use ntex::web::test;

    use super::*;
    // #[ntex::test]
    // async fn test_index_get() {
    //     let app = test::init_service(web::App::new().route("/", web::get().to(index))).await;
    //     let req = test::TestRequest::get().uri("/").to_request();
    //     let resp = test::call_service(&app, req).await;
    //     assert!(resp.status().is_success());
    // }
    //
    // #[ntex::test]
    // async fn test_index_post() {
    //     let app = test::init_service(web::App::new().route("/", web::get().to(index))).await;
    //     let req = test::TestRequest::post().uri("/").to_request();
    //     let resp = test::call_service(&app, req).await;
    //     assert!(resp.status().is_client_error());
    // }
}
// #[ntex::main]
// async fn main() -> std::io::Result<()> {
//     env_logger::init();
//
//     #[derive(OpenApi)]
//     #[openapi(
//     paths(
//     ),
//     components(
//     ),
//     tags(
//     ),
//     )]
//     struct ApiDoc;
//
//     // let store = Data::new(TodoStore::default());
//     // Make instance variable of ApiDoc so all worker threads gets the same instance.
//     let openapi = ApiDoc::openapi();
//
//     web::HttpServer::new(|| {
//         web::App::new()
//             .wrap(middleware::Logger::default())
//             .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
//                     // .service(Redoc::with_url("/redoc", openapi.clone()))
//                     // .service(
//                     //     SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
//                     // )
//                     // // There is no need to create RapiDoc::with_openapi because the OpenApi is served
//                     // // via SwaggerUi. Instead we only make rapidoc to point to the existing doc.
//                     // //
//                     // // If we wanted to serve the schema, the following would work:
//                     // // .service(RapiDoc::with_openapi("/api-docs/openapi2.json", openapi.clone()).path("/rapidoc"))
//                     // .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
//             .route(
//             "/",
//             web::get().to(|| async { web::HttpResponse::Ok().finish() }),
//         )
//
//     })
//         .bind((Ipv4Addr::UNSPECIFIED, 8080))?
//         .run()
//         .await;
// }
