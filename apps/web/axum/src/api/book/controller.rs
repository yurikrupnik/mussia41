use futures::TryStreamExt;
use super::model::{Book, BookListQuery, NewBook, UpdateBook};
// use super::model::{s}
use mongodb::bson::{doc, from_document};
use mongodb::Collection;
use axum::{
    extract::{Json, Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
};
use models::todo::Todo;
use proc_macros::DbResource;
use services::mongo::filter_and_options::construct_find_options_and_filter;
use services::mongo::service::{
    create_item, delete_by_id, drop_collection, get_by_id, update_by_id,
};
use shared::app_state::AppState;
// use shared::validation::validate_request_body;

/// Get a book by id
#[utoipa::path(
    get,
    path = "/api/book/{id}",
    tag = Book::TAG,
    responses(
        (status = 200, description = "Book found", body = Book),
        (status = 404, description = "Book not found", body = HttpError),
    ),
)]
pub async fn get_book(State(app_state): State<AppState>, id: Path<String>) -> impl IntoResponse {
    let item_id = id.0;
    let db = &app_state.db;
    // let query = query.into_inner();
    // let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let result = get_by_id::<Book>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => (StatusCode::OK, Json(&payload)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json(&"Newly created item is not found")).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(&err.to_string())).into_response()
    }
}

/// List all books
#[utoipa::path(
    get,
    path = "/api/book",
    tag = Book::TAG,
    params(BookListQuery),
    responses(
        (status = 200, description = "List of books", body = [Book]),
    ),
)]
pub async fn get_books(State(app_state): State<AppState>, query: Query<BookListQuery>) -> impl IntoResponse {
    let query = query.0;
    let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let db = &app_state.db;
    let collection: Collection<Book> = db.collection(Book::COLLECTION);
    let mut cursor = collection
        .find(filter, options)
        .await
        .expect("failed fetching");
    let mut payload: Vec<Book> = Vec::new();
    while let Some(item) = cursor
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        payload.push(item);
    }
    (StatusCode::OK, Json(payload)).into_response()
}

/// Create a new Book
#[utoipa::path(
    post,
    path = "/api/book",
    tag = Book::TAG,
    request_body = NewBook,
    responses(
    (status = 201, description = "Todo created", body = Todo),
    ),
)]
pub async fn create_book(body: Json<NewBook>, State(app_state): State<AppState>) -> impl IntoResponse {
    let db = &app_state.db;
    let body = body.0;
    // if let Err(response) = validate_request_body(&body) {
    //     return response; // Returns early if validation fails
    // }
    let response = create_item::<Book, NewBook>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Todo = from_document(payload).expect("error 5");
            (StatusCode::CREATED, Json(&doc)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, Json(&format!("No user found"))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(&err.to_string())).into_response(),
    }
}

/// Delete `Book` by ID
#[utoipa::path(
  delete,
  path = "/api/book/{id}",
  tag = Book::TAG,
  responses(
    (status = 200, description = "Book deleted", body = String),
    (status = 404, description = "Book not found", body = HttpError),
  ),
)]
pub async fn delete_book(State(app_state): State<AppState>, id: Path<String>) -> impl IntoResponse {
    let item_id = id.0;
    let db = &app_state.db;
    let result = delete_by_id::<Book>(db, &item_id).await;
    match result {
        Ok(delete_result) => {
            if delete_result.deleted_count == 1 {
                (StatusCode::OK, Json(&"successfully deleted!")).into_response()
            } else {
                (StatusCode::NOT_FOUND, Json(&format!("item with specified ID {item_id} not found!"))).into_response()
            }
        }
        Err(_) => {
            (StatusCode::NOT_FOUND, Json(&format!("item with specified ID {item_id} not found!"))).into_response()
        }
    }
}

/// Delete all `Book`
#[utoipa::path(
    delete,
    path = "/api/book",
    tag = Book::TAG,
    responses(
        (status = 200, description = "Book deleted", body = String),
    ),
)]
pub async fn drop_books(State(app_state): State<AppState>) -> impl IntoResponse {
    let db = &app_state.db;
    drop_collection::<Book>(db).await.expect("das");
    (StatusCode::OK, Json(&"successfully deleted!")).into_response()
}

/// Update a `Book` by id and `UpdateBook` struct.
#[utoipa::path(
    put,
    path = "/api/book/{id}",
    tag = Book::TAG,
    request_body = UpdateBook,
    responses(
    (status = 200, description = "Book updated", body = Book),
    (status = 404, description = "Book not found", body = HttpError),
    ),
)]
pub async fn update_book(
    id: Path<String>,
    app_state: State<AppState>,
    body: Json<UpdateBook>,
) -> impl IntoResponse {
    let body = body.0;
    // if let Err(response) = validate_request_body(&body) {
    //     return response; // Returns early if validation fails
    // }
    let item_id = id.0;
    let db = &app_state.db;
    let result = update_by_id::<Book, UpdateBook>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Todo = from_document(payload).unwrap();
            (StatusCode::OK, Json(&doc)).into_response()
        }
        None => (StatusCode::NOT_FOUND, Json(&format!("not found item with ID {item_id}"))).into_response(),
    }
}
