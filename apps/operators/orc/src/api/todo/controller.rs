use super::model::{NewTodo, Todo, TodoListQuery, Update};
use futures::TryStreamExt;
use mongodb::Collection;
use ntex::web::types::{Json, Path, Query, State};
use ntex::web::HttpResponse;
use proc_macros::DbResource;
use services::mongo::filter_and_options::construct_find_options_and_filter;
use services::mongo::service::{
    create_item, delete_by_id, drop_collection, get_by_id, update_by_id,
};
use shared::app_state::AppState;
use shared::validation::validate_request_body;

/// Get a todo by id
#[utoipa::path(
    get,
    path = "/todo/{id}",
    tag = "Todo",
    responses(
        (status = 200, description = "Todo found", body = Todo),
        (status = 404, description = "Todo not found", body = HttpError),
    ),
)]
pub async fn get_todo(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    // let query = query.into_inner();
    // let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    get_by_id::<Todo>(db, &item_id).await
}

/// List all todos
#[utoipa::path(
    get,
    path = "/todo",
    tag = "Todo",
    params(TodoListQuery),
    responses(
        (status = 200, description = "List of Todo", body = [Todo]),
    ),
)]
pub async fn get_todos(app_state: State<AppState>, query: Query<TodoListQuery>) -> HttpResponse {
    let query = query.into_inner();
    let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let db = &app_state.db;
    let collection: Collection<Todo> = db.collection(Todo::COLLECTION);
    let mut cursor = collection
        .find(filter, options)
        .await
        .expect("failed fetching");
    let mut payload: Vec<Todo> = Vec::new();
    while let Some(item) = cursor
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        payload.push(item);
    }
    HttpResponse::Ok().json(&payload)
}

/// Create a new todo
#[utoipa::path(
    post,
    path = "/todo",
    tag = "Todo",
    request_body = NewTodo,
    responses(
    (status = 201, description = "Todo created", body = Todo),
    ),
)]
pub async fn create_todo(body: Json<NewTodo>, app_state: State<AppState>) -> HttpResponse {
    let body = body.into_inner();
    let db = &app_state.db;
    create_item::<Todo, NewTodo>(db, body).await
}

/// Delete `Todo` by ID
#[utoipa::path(
  delete,
  path = "/todo/{id}",
  tag = "Todo",
  responses(
    (status = 200, description = "Todo deleted", body = String),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
pub async fn delete_todo(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    delete_by_id::<Todo>(db, &item_id).await
}

/// Delete all `Todo`
#[utoipa::path(
    delete,
    path = "/todo",
    tag = "Todo",
    responses(
        (status = 200, description = "Todo deleted", body = String),
    ),
)]
pub async fn drop_todos(app_state: State<AppState>) -> HttpResponse {
    let db = &app_state.db;
    drop_collection::<Todo>(db).await
}

/// Update a `Todo` by id and `Update` struct.
#[utoipa::path(
    put,
    path = "/todo/{id}",
    tag = "Todo",
    request_body = Update,
    responses(
    (status = 200, description = "Todo updated", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
    ),
)]
pub async fn update_todo(
    id: Path<String>,
    app_state: State<AppState>,
    body: Json<Update>,
) -> HttpResponse {
    let body = body.into_inner();
    let item_id = id.into_inner();
    let db = &app_state.db;
    if let Err(response) = validate_request_body(&body) {
        return response; // Returns early if validation fails
    }
    update_by_id::<Todo, Update>(db, body, &item_id).await
}