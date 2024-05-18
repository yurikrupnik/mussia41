mod api;
mod project;
mod services;
mod swagger;
mod users;

use api::routes;
use bb8::Pool;
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use general::socket_addrs::get_web_url;
use general::{get_mongo_uri, get_redis_uri};
use mongodb::Client;
use ntex::web::{self, App, HttpResponse};
use shared::app_state::AppState;
use swagger::ApiDoc;

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
            .configure(routes)
            .default_service(web::route().to(default))
    })
    .bind(get_web_url(false))?
    .workers(1)
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::api::todo::controller::get_todo;
    use ntex::web::test;

    // #[ntex::test]
    // async fn test_index_ok() {
    //     let req = test::TestRequest::default()
    //         .header("content-type", "text/plain")
    //         .uri("/api/todo")
    //         .to_http_request();
    //     let resp = get_todo(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);
    // }

    // #[ntex::test]
    // async fn test_index_not_ok() {
    //     let req = test::TestRequest::default().to_http_request();
    //     let resp = get_todo(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    // }
}
