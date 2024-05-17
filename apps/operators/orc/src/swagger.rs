use super::api::todo::controller as todo;
use super::api::todo::model::{NewTodo, Todo, Update};
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
    ),
    components(schemas(HttpError, User, UpdateUser, NewUser, Todo, NewTodo, Update))
)]
pub(crate) struct ApiDoc;
