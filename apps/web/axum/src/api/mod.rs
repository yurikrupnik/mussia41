use axum::Router;

use book::router::router as book_router;
// use login::router::router as login_router;
use shared::app_state::AppState;
use todo::router::router as todo_router;

pub mod book;
pub mod todo;
mod users;
mod login;

pub fn routes() -> Router<AppState> {
  Router::new()
    // .nest("", login_router())
    .nest("/api", todo_router())
    .nest("/api", book_router())
  // .merge(todo_router())
}
