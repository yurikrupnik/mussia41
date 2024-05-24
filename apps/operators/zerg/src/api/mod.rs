use ntex::web;

pub mod namespace;
// pub mod todo;
// pub mod shit;

use crate::api::namespace::config::routes as namespace_routes;
// use crate::api::todo::config::todo_routes;
// use crate::api::shit::config::shit_routes;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(namespace_routes)
            // .configure(book_routes)
            // .configure(shit_routes)
        // .configure(services_routes)
        // .configure(generic_routes::<User, NewUser, UpdateUser, UserListQuery>)
        // .configure(generic_routes::<Project, NewProject, UpdateProject, ProjectListQuery>),
    );
}
