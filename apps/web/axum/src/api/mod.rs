use axum::Router;
use shared::app_state::AppState;

pub mod todo;
pub mod book;

use todo::router::router as todo_router;
use book::router::router as book_router;

pub fn routes() -> Router<AppState>  {
    Router::new()
        .nest("/api", todo_router())
        .nest("/api", book_router())
        // .merge(todo_router())
}