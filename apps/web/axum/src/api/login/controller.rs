use axum::{
  extract::{Json, Path, State},
  http::StatusCode,
  response::IntoResponse,
};

use models::todo::Todo;
use services::mongo::service::get_by_id;
use shared::app_state::AppState;

/// Login
#[utoipa::path(
  post,
  path = "/api/login",
  tag = "Login",
  responses(
    (status = 200, description = "User logged in successfully", body = Todo),
    (status = 404, description = "Todo not found", body = HttpError),
  ),
)]
pub async fn get_todo(State(app_state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
  // let item_id = id;
  let db = &app_state.db;

  let result = get_by_id::<Todo>(db, &id).await;
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
