
pub mod todo;
// pub mod book;
// mod book;

use axum::Router;
use axum::routing::post;

pub fn routes() -> Router<()>  {
    let todo_routes = Router::new().route("/todo", post(|| async {}));
    let todso_routes = Router::new().route("/todoa", post(|| async {}));
    let s = Router::new().nest("/api", todo_routes).merge(todso_routes);
    s
    // cfg.service(
    //     scope("/api")
    //         .configure(todo_configure)
    //         .configure(book_configure)
    // );
}