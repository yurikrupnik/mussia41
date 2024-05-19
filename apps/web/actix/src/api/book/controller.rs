use mongodb::Collection;
use models::book::{NewBook, Book, BookListQuery, UpdateBook};
use services::mongo::filter_and_options::construct_find_options_and_filter;
use shared::app_state::AppState;
use proc_macros::DbResource;
use actix_web::{web::{Query, Data, Json, Path}, HttpResponse};
use futures::TryStreamExt;
use mongodb::bson::from_document;
use services::mongo::service::{create_item, delete_by_id, drop_collection, get_by_id, update_by_id};
// use shared::validation::validate_request_body;

/// Get a Book by id
#[utoipa::path(
    get,
    path = "/api/book/{id}",
    tag = Book::TAG,
    responses(
    (status = 200, description = "Book found", body = Book),
    (status = 404, description = "Book not found", body = HttpError),
    ),
)]
pub async fn get_book(app_state: Data<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    // let query = query.into_inner();
    // let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let result = get_by_id::<Book>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => HttpResponse::Ok().json(&payload),
        Ok(None) => HttpResponse::NotFound().json(&format!("No item found with id {item_id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// List all books
#[utoipa::path(
    get,
    path = "/api/book",
    tag = Book::TAG,
    params(BookListQuery),
    responses(
    (status = 200, description = "List of book", body = [book]),
    ),
)]
pub async fn get_books(app_state: Data<AppState>, query: Query<BookListQuery>) -> HttpResponse {
    let query = query.into_inner();
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
    HttpResponse::Ok().json(&payload)
}

// Delete `Book` by ID
#[utoipa::path(
    delete,
    path = "/api/book/{id}",
    tag = Book::TAG,
    responses(
    (status = 200, description = "Book deleted", body = String),
    (status = 404, description = "Book not found", body = HttpError),
    ),
)]
pub async fn delete_book(app_state: Data<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = delete_by_id::<Book>(db, &item_id).await;
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

/// Create a new Book
#[utoipa::path(
    post,
    path = "/api/book",
    tag = Book::TAG,
    request_body = Newbook,
    responses(
    (status = 201, description = "book created", body = book),
    ),
)]
pub async fn create_book(body: Json<NewBook>, app_state: Data<AppState>) -> HttpResponse {
    let body = body.into_inner();
    let db = &app_state.db;
    // if let Err(response) = validate_request_body(&body) {
    //     return response; // Returns early if validation fails
    // }
    let response = create_item::<Book, NewBook>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Book = from_document(payload).expect("error 5");
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => HttpResponse::NotFound().json(&format!("No user found with id")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// Delete all `Book`
#[utoipa::path(
    delete,
    path = "/api/book",
    tag = Book::TAG,
    responses(
    (status = 200, description = "book deleted", body = String),
    ),
)]
pub async fn drop_books(app_state: Data<AppState>) -> HttpResponse {
    let db = &app_state.db;
    drop_collection::<Book>(db).await.expect("das");
    HttpResponse::Ok().body("successfully deleted!")
}

/// Update a `Book` by id and `Update` struct.
#[utoipa::path(
    put,
    path = "/api/book/{id}",
    tag = Book::TAG,
    request_body = UpdateBook,
    responses(
    (status = 200, description = "book updated", body = Book),
    (status = 404, description = "book not found", body = HttpError),
    ),
)]
pub async fn update_book(
    id: Path<String>,
    app_state: Data<AppState>,
    body: Json<UpdateBook>,
) -> HttpResponse {
    let body = body.into_inner();

    //     .map_err(|e| HttpResponse::BadRequest().body(e.to_string())).unwrap();
    // if let Err(response) = validate_request_body(&body) {
    //     return response; // Returns early if validation fails
    // }
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = update_by_id::<Book, UpdateBook>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Book = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        None => HttpResponse::NotFound().json(&format!("not found item with ID {item_id}")),
    }
}
