use axum::{Router, routing};
use shared::app_state::AppState;
use proc_macros::DbResource;
use super::model::Todo;
use super::controller::{get_todos, create_todo, delete_todo, update_todo, get_todo, drop_todos};

/// Todo endpoint router - includes full CRUD
pub fn router() -> Router<AppState> {
    Router::new()
        .route(Todo::URL, routing::get(get_todos).delete(drop_todos).post(create_todo))
        .route(&format!("{}/:id", Todo::URL), routing::put(update_todo).get(get_todo).delete(delete_todo))
}