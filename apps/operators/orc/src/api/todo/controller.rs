use super::model::{NewTodo, Todo, TodoListQuery, Update};
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document};
use mongodb::Collection;
use ntex::web::types::{Json, Path, Query, State};
use ntex::web::{resource, HttpResponse};
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
    path = "/api/todo/{id}",
    tag = Todo::TAG,
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
    let result = get_by_id::<Todo>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => HttpResponse::Ok().json(&payload),
        Ok(None) => HttpResponse::NotFound().json(&format!("No item found with id {item_id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// List all todos
#[utoipa::path(
    get,
    path = "/api/todo",
    tag = Todo::TAG,
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
    path = "/api/todo",
    tag = Todo::TAG,
    request_body = NewTodo,
    responses(
    (status = 201, description = "Todo created", body = Todo),
    ),
)]
pub async fn create_todo(body: Json<NewTodo>, app_state: State<AppState>) -> HttpResponse {
    let body = body.into_inner();
    let db = &app_state.db;
    if let Err(response) = validate_request_body(&body) {
        return HttpResponse::BadRequest().json(&response);
    }
    let response = create_item::<Todo, NewTodo>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Todo = from_document(payload).expect("error 5");
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => HttpResponse::NotFound().json::<String>(&format!("No user found with id")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// Delete `Todo` by ID
#[utoipa::path(
  delete,
  path = "/api/todo/{id}",
  tag = Todo::TAG,
  responses(
    (status = 200, description = "Todo deleted", body = String),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
pub async fn delete_todo(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = delete_by_id::<Todo>(db, &item_id).await;
    match result {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                HttpResponse::Ok().json(&"successfully deleted!")
            } else {
                HttpResponse::NotFound()
                    .json(&format!("item with specified ID {item_id} not found!"))
            }
        }
        Err(_) => {
            HttpResponse::NotFound().json(&format!("item with specified ID {item_id} not found!"))
        }
    }
}

/// Delete all `Todo`
#[utoipa::path(
    delete,
    path = "/api/todo",
    tag = Todo::TAG,
    responses(
        (status = 200, description = "Todo deleted", body = String),
    ),
)]
pub async fn drop_todos(app_state: State<AppState>) -> HttpResponse {
    let db = &app_state.db;
    drop_collection::<Todo>(db).await.expect("das");
    HttpResponse::Ok().body("successfully deleted!")
}

/// Update a `Todo` by id and `Update` struct.
#[utoipa::path(
    put,
    path = "/api/todo/{id}",
    tag = Todo::TAG,
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
    if let Err(response) = validate_request_body(&body) {
        return HttpResponse::BadRequest().json(&response);
    }
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = update_by_id::<Todo, Update>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Todo = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        None => HttpResponse::NotFound().json(&format!("not found item with ID {item_id}")),
    }
}
