use super::controller::{handle_request, create_shit, delete_shit, drop_shits, get_shit, get_shits, update_shit};
use ntex::web;

pub fn shit_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/shit").service((
            web::resource("")
                .route(web::delete().to(drop_shits))
                .route(web::get().to(get_shits))
                .route(web::post().to(handle_request)),
            web::resource("/{id}")
                .route(web::get().to(get_shit))
                .route(web::put().to(update_shit))
                .route(web::delete().to(delete_shit)),
        )),
    );
}
