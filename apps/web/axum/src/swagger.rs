use utoipa::OpenApi;

// use models::book::{Book, NewBook, UpdateBook};
use services::errors::ntex::HttpError;

use super::api::{
  book::{
    controller as book,
    model::{Book, NewBook, UpdateBook},
  },
  // project::model::{NewProject, Project, UpdateProject},
  // services::{
  //     controller as service,
  //     model::{NewService, Service, UpdateService},
  // },
  todo::{
    controller as todo,
    model::{NewTodo, Todo, Update},
  },
  // users::model::NewUser,
};

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
  paths(
    todo::get_todos,
    todo::create_todo,
    todo::get_todo,
    todo::update_todo,
    todo::delete_todo,
    todo::drop_todos,
    book::get_book,
    book::get_books,
    book::create_book,
    book::update_book,
    book::delete_book,
    book::drop_books,
  // service::get_service,
  // service::get_services,
  // service::create_service,
  // service::update_service,
  // service::delete_service,
  // service::drop_services,
  // project::get_project,
  // project::get_projects,
  // project::create_project,
  // project::update_project,
  // project::delete_project,
  // project::drop_projects,
  ),
  components(schemas(
    HttpError,
  // User,
  // UpdateUser,
  // NewUser,
    Todo,
    NewTodo,
    Update,
  // Service,
  // UpdateService,
  // NewService,
    Book,
    UpdateBook,
    NewBook,
  // NewProject,
  // Project,
  // UpdateProject
  ))
)]
pub struct ApiDoc;
