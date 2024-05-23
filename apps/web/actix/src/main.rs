mod api;
mod swagger;

use std::net::Ipv4Addr;
use actix_web::{
    middleware::Logger,
    get, web, App, HttpResponse, HttpServer, Responder
};
use general::get_port;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use shared::app_state::AppState;
use api::routes;
use services::{
    mongo::connector::connect as mongo_connect,
    redis::connector::connect as redis_connect
};
use swagger::ApiDoc;
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

    // let client = Client::with_uri_str(get_mongo_uri())
    //     .await
    //     .expect("failed to connect!");
    // let db = client.database("aris");
    let db = mongo_connect("aris").await;
    // let manager = RedisConnectionManager::new(get_redis_uri()).unwrap();
    // let redis_pool = Pool::builder().build(manager).await.unwrap();
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
