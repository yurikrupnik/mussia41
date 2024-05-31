mod api;
mod swagger;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use api::routes;
use general::get_port;
use services::{
    mongo::connector::connect as mongo_connect, redis::connector::connect as redis_connect,
};
use shared::app_state::AppState;
use std::net::Ipv4Addr;
use swagger::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!?!!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = mongo_connect("aris").await;
    let redis_pool = redis_connect().await;
    let state = AppState::new(db, redis_pool);

    log::info!("Starting HTTP server on http://localhost:{}!", get_port());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .service(hello)
            .route("/hey", web::get().to(manual_hello))
            .configure(routes)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .default_service(web::to(HttpResponse::NotFound))
    })
    .bind((Ipv4Addr::UNSPECIFIED, get_port()))?
    .run()
    .await
}
