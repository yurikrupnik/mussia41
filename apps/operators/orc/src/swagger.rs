use super::api::todo::controller as todo;
use super::api::todo::model::{NewTodo, Todo, Update};
use super::api::services::controller as service;
use super::api::services::model::{Service, UpdateService, NewService};
use super::services::error::HttpError;
use super::users::model::{NewUser, UpdateUser, User};
use utoipa::OpenApi;

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
        service::get_service,
        service::get_services,
        service::create_service,
        service::update_service,
        service::delete_service,
        service::drop_services,
    ),
    components(schemas(HttpError, 
        User, UpdateUser, NewUser, Todo, NewTodo, Update, Service, UpdateService, NewService))
)]
pub(crate) struct ApiDoc;
