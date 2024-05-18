use crate::api::todo::config::todo_routes;
use generic_api::config::generic_routes;
use ntex::web;
// use crate::api::book::config::;
use crate::api::book::config::book_routes;
use crate::api::services::config::services_routes;
use crate::users::model::{NewUser, UpdateUser, User, UserListQuery};

pub mod book;
pub mod services;
pub mod todo;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(todo_routes)
            .configure(services_routes)
            .configure(book_routes)
            .configure(generic_routes::<User, NewUser, UpdateUser, UserListQuery>), // .wrap(middleware::froms_fn(validate_body))
                                                                                    // .service((
                                                                                    //     web::resource("")
                                                                                    //         .route(web::delete().to(drop_todos))
                                                                                    //         .route(web::get().to(get_todos))
                                                                                    //         .route(web::post().to(create_todo)),
                                                                                    //     web::resource("/{id}")
                                                                                    //         .route(web::get().to(get_todo))
                                                                                    //         .route(web::put().to(update_todo))
                                                                                    //         .route(web::delete().to(delete_todo)),
                                                                                    // )),
    );
}
