mod api;
use bb8::Pool;
use bb8_redis::bb8;
use bb8_redis::RedisConnectionManager;
use axum::{
    routing::{get, post},
    Router,
    response::{Html},
};
use general::socket_addrs::get_web_url_v1;
use mongodb::Client;
// use tracing::{info, Level};
// use tracing_subscriber::FmtSubscriber;
use general::{get_mongo_uri, get_redis_uri};
use shared::app_state::AppState;
use api::todo::{controller::{create_todo, get_todo, get_todos, delete_todo, drop_todos, update_todo}};
// use api::book::{controller::{create_book, get_book, get_books, delete_book, drop_books, update_book}};
// use std::time::Duration;
// use tokio::net::TcpListener;
// use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
// use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    // Set up logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::INFO)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();
    let client = Client::with_uri_str(get_mongo_uri())
        .await
        .expect("failed to connect");
    let db = client.database("aris");

    let manager = RedisConnectionManager::new(get_redis_uri()).unwrap();
    let redis_pool = Pool::builder().build(manager).await.unwrap();
    let state = AppState::new(db, redis_pool);
    // log::info!("Starting HTTP server on http://localhost:{}!", get_port());
    let todo_routes = Router::new()
        .route("/todo", get(get_todos).delete(drop_todos).post(create_todo))
        .route("/todo/:id", get(get_todo).delete(delete_todo).put(update_todo));
    // let book_routes = Router::new()
    //     .route("/book", get(get_books).delete(drop_books).post(create_book))
    //     .route("/book/:id", get(get_book).delete(delete_book).put(update_book));
    // let todso_routes = Router::new().route("/book", get(|| async {}));
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        // .merge(app)
        .merge(todo_routes)
        // .merge(book_routes)
        .with_state(state);

    // run our app with hyper, listening globally on port
    let listener_value = get_web_url_v1();
    let listener = tokio::net::TcpListener::bind(listener_value).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
