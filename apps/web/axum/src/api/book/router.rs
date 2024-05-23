pub use axum::{Router, routing};
use shared::app_state::AppState;
use proc_macros::DbResource;
use models::book::Book;
use super::controller::{get_books, create_book, get_book, update_book, delete_book, drop_books};

/// Book router - includes full CRUD
pub fn router() -> Router<AppState>  {
    Router::new()
        .route(Book::URL, routing::get(get_books).delete(drop_books))
        .route(&format!("{}/:id", Book::URL), routing::put(update_book).get(get_book).delete(delete_book))
}