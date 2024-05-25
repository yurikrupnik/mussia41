use super::api::namespace::{
    controller as namespace,
    model::{Namespace, NewNamespace, UpdateNamespace},
};
use services::errors::ntex::HttpError;
use utoipa::OpenApi;

/// Main structure to generate OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(
        namespace::list_namespaces,
        namespace::create_namespace,
        // todo::get_todos,
        // todo::create_todo,
        // todo::get_todo,
        // todo::update_todo,
        // todo::delete_todo,
        // todo::drop_todos,
        // service::get_service,
        // service::get_services,
        // service::create_service,
        // service::update_service,
        // service::delete_service,
        // service::drop_services,
        // book::get_book,
        // book::get_books,
        // book::create_book,
        // book::update_book,
        // book::delete_book,
        // book::drop_books,
        // project::get_project,
        // project::get_projects,
        // project::create_project,
        // project::update_project,
        // project::delete_project,
        // project::drop_projects,
    ),
    components(schemas(
        HttpError,
        Namespace,
        NewNamespace,
        UpdateNamespace
        // User,
        // UpdateUser,
        // NewUser,
        // Todo,
        // NewTodo,
        // Update,
        // Service,
        // UpdateService,
        // NewService,
        // Book,
        // UpdateBook,
        // NewBook,
        // NewProject,
        // Project,
        // UpdateProject
    ))
)]
pub(crate) struct ApiDoc;
