use axum::Router;
use axum::routing::post;
use super::controller::{get_todos, create_todo};


// pub fn routes() -> Router<()>  {
    // Router::new().route("/", post(create_todo))
    // cfg.service(
    //     scope("/api")
    //         .configure(todo_configure)
    //         .configure(book_configure)
    // );
// }