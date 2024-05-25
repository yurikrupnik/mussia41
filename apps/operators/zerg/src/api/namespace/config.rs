use super::controller::{create_namespace, delete_namespace, list_namespaces, update_namespace};
use ntex::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/namespace").service((
            web::resource("")
                // .route(web::delete().to(drop_todos))
                .route(web::get().to(list_namespaces))
                .route(web::post().to(create_namespace)),
            web::resource("/{id}")
                //     .route(web::get().to(get_todo))
                .route(web::put().to(update_namespace))
                .route(web::delete().to(delete_namespace)),
        )),
    );
}
