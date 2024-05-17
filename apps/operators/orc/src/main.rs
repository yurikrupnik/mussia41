mod api;
mod project;
mod services;
mod swagger;
mod users;

use bb8::Pool;
use bb8_redis::bb8;
use generic_api::config::generic_routes;
// use project::model::{NewProject, Project, ProjectListQuery, UpdateProject};
use api::services::model::{NewService, Service, ServiceListQuery, UpdateService};
use api::todo::config::todo_routes;
use users::model::{NewUser, UpdateUser, User, UserListQuery};
// use users::model::{NewUser, UpdateProfile, User};
// use bb8_redis::redis::AsyncCommands;
use bb8_redis::RedisConnectionManager;
use general::socket_addrs::get_web_url;
use general::{get_mongo_uri, get_redis_uri};
use mongodb::Client;
use ntex::web::{self, HttpResponse, Responder};
use shared::app_state::AppState;
use swagger::ApiDoc;

#[web::get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!")
}

async fn default() -> HttpResponse {
    HttpResponse::NotFound().finish()
}
#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let client = Client::with_uri_str(get_mongo_uri())
        .await
        .expect("failed to connect");
    let db = client.database("aris");

    let manager = RedisConnectionManager::new(get_redis_uri()).unwrap();
    let redis_pool = Pool::builder().build(manager).await.unwrap();

    let state = AppState::new(db, redis_pool);

    web::HttpServer::new(move || {
        let json_config = web::types::JsonConfig::default().limit(4096);
        web::App::new()
            // Register swagger endpoints
            .configure(services::openapi::ntex_config::<ApiDoc>)
            .wrap(web::middleware::Logger::default())
            .state(json_config)
            .state(state.clone())
            .service(hello)
            .configure(todo_routes)
            // .configure(generic_routes::<Project, NewProject, UpdateProject, ProjectListQuery>)
            .configure(generic_routes::<Service, NewService, UpdateService, ServiceListQuery>)
            .configure(generic_routes::<User, NewUser, UpdateUser, UserListQuery>)
            .default_service(web::route().to(default))
    })
    .bind(get_web_url(false))?
    .workers(1)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    // use ntex::web;
    // use ntex::web::test;
    //
    // use super::*;
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
