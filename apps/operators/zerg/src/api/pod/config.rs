use super::controller::{create_pod, list_pods};
use ntex::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pod").service((
            web::resource("")
                // .route(web::delete().to(drop_todos))
                .route(web::get().to(list_pods))
                .route(web::post().to(create_pod)),
            // web::resource("/{id}")
            //     .route(web::get().to(get_todo))
            //     .route(web::put().to(update_todo))
            //     .route(web::delete().to(delete_todo)),
        )),
    );
}
