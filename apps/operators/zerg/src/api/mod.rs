use ntex::web;

pub mod namespace;
// pub mod pod;

use crate::api::namespace::config::routes as namespace_routes;
// use crate::api::pod::config::routes as pod_routes;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").configure(namespace_routes), // .configure(pod_routes),
    );
}
