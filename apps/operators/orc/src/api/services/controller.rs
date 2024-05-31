use super::model::{NewService, Service, ServiceListQuery, UpdateService};
use futures::TryStreamExt;
use mongodb::bson::{doc, from_document};
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

/// Get a `Service` by id
#[utoipa::path(
    get,
    path = "/api/service/{id}",
    tag = Service::TAG,
    responses(
        (status = 200, description = "Service found", body = Service),
        (status = 404, description = "Service not found", body = HttpError),
    ),
)]
pub async fn get_service(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    // let query = query.into_inner();
    // let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let result = get_by_id::<Service>(db, &item_id).await;
    match result {
        Ok(Some(payload)) => HttpResponse::Ok().json(&payload),
        Ok(None) => HttpResponse::NotFound().json(&format!("No item found with id {item_id}")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// List all services
#[utoipa::path(
    get,
    path = "/api/service",
    tag = Service::TAG,
    params(ServiceListQuery),
    responses(
        (status = 200, description = "List of Service", body = [Service]),
    ),
)]
pub async fn get_services(
    app_state: State<AppState>,
    query: Query<ServiceListQuery>,
) -> HttpResponse {
    let query = query.into_inner();
    let (filter, options) = construct_find_options_and_filter(query.clone()).unwrap();
    let db = &app_state.db;
    let collection: Collection<Service> = db.collection(Service::COLLECTION);
    let mut cursor = collection
        .find(filter, options)
        .await
        .expect("failed fetching");
    let mut payload: Vec<Service> = Vec::new();
    while let Some(item) = cursor
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        payload.push(item);
    }
    HttpResponse::Ok().json(&payload)
}

/// Create a new service
#[utoipa::path(
    post,
    path = "/api/service",
    tag = Service::TAG,
    request_body = NewService,
    responses(
    (status = 201, description = "Service created", body = Service),
    ),
)]
pub async fn create_service(body: Json<NewService>, app_state: State<AppState>) -> HttpResponse {
    let body = body.into_inner();
    let db = &app_state.db;
    // if let Err(response) = validate_request_body(&body) {
    //   return HttpResponse::Bad().json(&response);
    // }
    let response = create_item::<Service, NewService>(db, body).await;
    match response {
        Ok(Some(payload)) => {
            let doc: Service = from_document(payload).expect("error 5");
            HttpResponse::Created().json(&doc)
        }
        Ok(None) => HttpResponse::NotFound().json::<String>(&format!("No user found with id")),
        Err(err) => HttpResponse::InternalServerError().json(&err.to_string()),
    }
}

/// Delete `Service` by ID
#[utoipa::path(
  delete,
  path = "/api/service/{id}",
  tag = Service::TAG,
  responses(
    (status = 200, description = "Service deleted", body = String),
    (status = 404, description = "Service not found", body = HttpError),
  ),
)]
pub async fn delete_service(app_state: State<AppState>, id: Path<String>) -> HttpResponse {
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = delete_by_id::<Service>(db, &item_id).await;
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

/// Delete all `Service`
#[utoipa::path(
    delete,
    path = "/api/service",
    tag = Service::TAG,
    responses(
        (status = 200, description = "Service deleted", body = String),
    ),
)]
pub async fn drop_services(app_state: State<AppState>) -> HttpResponse {
    let db = &app_state.db;
    drop_collection::<Service>(db).await.expect("das");
    HttpResponse::Ok().body("successfully deleted!")
}

/// Update a `Service` by id and `UpdateService` struct.
#[utoipa::path(
    put,
    path = "/api/service/{id}",
    tag = Service::TAG,
    request_body = UpdateService,
    responses(
    (status = 200, description = "Service updated", body = Service),
    (status = 404, description = "Service not found", body = HttpError),
    ),
)]
pub async fn update_service(
    id: Path<String>,
    app_state: State<AppState>,
    body: Json<UpdateService>,
) -> HttpResponse {
    let body = body.into_inner();
    if let Err(response) = validate_request_body(&body) {
        return response; // Returns early if validation fails
    }
    let item_id = id.into_inner();
    let db = &app_state.db;
    let result = update_by_id::<Service, UpdateService>(db, body, &item_id)
        .await
        .unwrap();
    match result {
        Some(payload) => {
            let doc: Service = from_document(payload).unwrap();
            HttpResponse::Created().json(&doc)
        }
        None => HttpResponse::NotFound().json(&format!("not found item with ID {item_id}")),
    }
}
