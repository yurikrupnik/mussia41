use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use futures::TryStreamExt;
use models::todo::{NewTodo, Todo, TodoListQuery, Update};
use mongodb::bson::from_document;
use mongodb::Collection;
use proc_macros::DbResource;
use services::mongo::filter_and_options::construct_find_options_and_filter;
use services::mongo::service::{
    create_item, delete_by_id, drop_collection, get_by_id, update_by_id,
};
use shared::app_state::AppState;
use shared::validation::validate_request_body;
use validator::{Validate, ValidationErrors};

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
pub async fn get_todo(State(app_state): State<AppState>, id: Path<String>) -> impl IntoResponse {
    let item_id = id.0;
    let db = &app_state.db;

    let result = get_by_id::<Todo>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => (StatusCode::OK, Json(&payload)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(&"Newly created item is not found"),
        )
            .into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(&err.to_string())).into_response(),
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
pub async fn get_todos(
    State(app_state): State<AppState>,
    query: Query<TodoListQuery>,
) -> impl IntoResponse {
    let query = query.0;
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
    (StatusCode::OK, Json(payload)).into_response()
}

// Delete `Todo` by ID
#[utoipa::path(
    delete,
    path = "/api/todo/{id}",
    tag = Todo::TAG,
    responses(
    (status = 200, description = "Todo deleted", body = String),
    (status = 404, description = "Todo not found", body = HttpError),
    ),
)]
pub async fn delete_todo(State(app_state): State<AppState>, id: Path<String>) -> impl IntoResponse {
    let item_id = id.0;
    let db = &app_state.db;
    let result = delete_by_id::<Todo>(db, &item_id).await;
    match result {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                (StatusCode::OK, Json(&"successfully deleted!")).into_response()
                // HttpResponse::Ok().json(&"successfully deleted!")
            } else {
                (
                    StatusCode::NOT_FOUND,
                    Json(&format!("item with specified ID {item_id} not found!")),
                )
                    .into_response()
                // HttpResponse::NotFound()
                //     .json(&format!("item with specified ID {item_id} not found!"))
            }
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(&format!("item with specified ID {item_id} not found!")),
        )
            .into_response(),
    }
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
pub async fn create_todo(
    State(app_state): State<AppState>,
    body: Json<NewTodo>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let body = body.0;
    // let s = body.create();
    if let Err(e) = body.validate() {
        return (StatusCode::BAD_REQUEST, Json(&e)).into_response();
    }
    //     .map_err(|e| (StatusCode::BAD_REQUEST, Json(&e)).into_response())
    //     .expect("TODO: panic message");
    if let Err(errors) = validate_request_body(&body) {
        return (StatusCode::BAD_REQUEST, Json(&errors)).into_response();
    }
    let response = create_item::<Todo, NewTodo>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Todo = from_document(payload).expect("error 5");
            (StatusCode::CREATED, Json(&doc)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(&format!("No user found"))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(&err.to_string())).into_response(),
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
pub async fn drop_todos(State(app_state): State<AppState>) -> impl IntoResponse {
    let db = &app_state.db;
    drop_collection::<Todo>(db).await.expect("das");
    (StatusCode::OK, Json(&"successfully deleted!")).into_response()
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
    State(app_state): State<AppState>,
    body: Json<Update>,
) -> impl IntoResponse {
    let body = body.0;
    if let Err(errors) = validate_request_body(&body) {
        return (StatusCode::BAD_REQUEST, Json(&errors)).into_response();
    }
    let item_id = id.0;
    let db = &app_state.db;
    let result = update_by_id::<Todo, Update>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Todo = from_document(payload).unwrap();
            (StatusCode::OK, Json(&doc)).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(&format!("not found item with ID {item_id}")),
        )
            .into_response(),
    }
}
