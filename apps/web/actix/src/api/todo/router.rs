// use super::handlers::*;
// use super::model::Todo;
use actix_web::web::{delete, get, post, put, resource, scope, ServiceConfig};
use proc_macros::DbResource;
use super::model::Todo;
use super::controller::{get_todos, update_todo, delete_todo, drop_todos, get_todo, create_todo};

pub fn todo_configure(cfg: &mut ServiceConfig) {
    cfg.service(
        scope(Todo::URL)
            .service(
                resource("")
                    .route(get().to(get_todos))
                    .route(delete().to(drop_todos))
                    .route(post().to(create_todo)),
            )
            .service(
                resource("/{id}")
                    .route(delete().to(delete_todo))
                    .route(put().to(update_todo))
                    .route(get().to(get_todo)),
            ),
    );
}
