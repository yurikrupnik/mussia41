use crate::api::book::router::book_configure;
use crate::api::todo::router::todo_configure;
use actix_web::web::{scope, ServiceConfig};
pub mod book;
pub mod todo;
// pub mod users;

pub fn routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .configure(todo_configure)
            .configure(book_configure),
    );
}
