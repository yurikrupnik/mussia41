use super::controller::{
    create_service, delete_service, drop_services, get_service, get_services, update_service,
};
use super::model::Service;
use ntex::web;
use proc_macros::DbResource;

pub fn services_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope(Service::URL).service((
            web::resource("")
                .route(web::delete().to(drop_services))
                .route(web::get().to(get_services))
                .route(web::post().to(create_service)),
            web::resource("/{id}")
                .route(web::get().to(get_service))
                .route(web::put().to(update_service))
                .route(web::delete().to(delete_service)),
        )),
    );
}
