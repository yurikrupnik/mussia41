use super::controller::{create_book, delete_book, drop_books, get_book, get_books, update_book};
use models::book::Book;
use ntex::web;
use proc_macros::DbResource;

pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(Book::URL).service((
            web::resource("")
                .route(web::delete().to(drop_books))
                .route(web::get().to(get_books))
                .route(web::post().to(create_book)),
            web::resource("/{id}")
                .route(web::get().to(get_book))
                .route(web::put().to(update_book))
                .route(web::delete().to(delete_book)),
        )),
    );
}
