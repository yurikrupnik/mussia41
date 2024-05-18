use crate::api::todo::config::todo_routes;
use generic_api::config::generic_routes;
use ntex::web;
use crate::api::book::config::book_routes;
use crate::api::services::config::services_routes;
use crate::api::users::model::{NewUser, UpdateUser, User, UserListQuery};
use crate::api::project::model::{NewProject, UpdateProject, Project, ProjectListQuery};

pub mod book;
pub mod services;
pub mod todo;
pub mod users;
pub mod project;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(todo_routes)
            .configure(services_routes)
            .configure(book_routes)
            .configure(generic_routes::<User, NewUser, UpdateUser, UserListQuery>)
            .configure(generic_routes::<Project, NewProject, UpdateProject, ProjectListQuery>),
    );
}
