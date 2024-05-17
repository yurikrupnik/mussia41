use super::controller::{create_todo, delete_todo, drop_todos, get_todo, get_todos, update_todo};
use ntex::web;

pub fn todo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            // .wrap(middleware::from_fn(validate_body))
            .service((
                web::resource("")
                    .route(web::delete().to(drop_todos))
                    .route(web::get().to(get_todos))
                    .route(web::post().to(create_todo)),
                web::resource("/{id}")
                    .route(web::get().to(get_todo))
                    .route(web::put().to(update_todo))
                    .route(web::delete().to(delete_todo)),
            )),
    );
}
