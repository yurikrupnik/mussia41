use super::controller::{create_book, delete_book, drop_books, get_book, get_books, update_book};
pub use axum::{routing, Router};
use models::book::Book;
use proc_macros::DbResource;
use shared::app_state::AppState;

/// Book router - includes full CRUD
pub fn router() -> Router<AppState> {
    Router::new()
        .route(Book::URL, routing::get(get_books).delete(drop_books))
        .route(
            &format!("{}/:id", Book::URL),
            routing::put(update_book).get(get_book).delete(delete_book),
        )
}
