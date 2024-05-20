use super::api::{
    book::controller as book,
    todo::{
        controller as todo,
        model::{NewTodo, Todo, Update},
    },
};
use models::book::{Book, NewBook, UpdateBook};
use services::errors::ntex::HttpError;
use utoipa::OpenApi;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        todo::get_todo,
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
    ),
    components(schemas(
        HttpError,
        Todo,
        NewTodo,
        Update,
        Book,
        UpdateBook,
        NewBook,
    ))
)]
pub(crate) struct ApiDoc;
