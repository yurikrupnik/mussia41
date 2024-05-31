use ntex::web;

pub mod book;
pub mod shit;
pub mod todo;

use crate::api::book::config::book_routes;
use crate::api::shit::config::shit_routes;
use crate::api::todo::config::todo_routes;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(todo_routes)
            .configure(book_routes)
            .configure(shit_routes), // .configure(services_routes)
                                     // .configure(generic_routes::<User, NewUser, UpdateUser, UserListQuery>)
                                     // .configure(generic_routes::<Project, NewProject, UpdateProject, ProjectListQuery>),
    );
}
