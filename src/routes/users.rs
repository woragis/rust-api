use crate::controllers::users::{
    create_user_handler, delete_user_handler, read_user_handler, read_users_handler,
    update_user_handler,
};
use actix_web::web;

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/", web::post().to(create_user_handler))
            .route("/", web::get().to(read_users_handler))
            .route("/id}", web::get().to(read_user_handler))
            .route("/id}", web::put().to(update_user_handler))
            .route("/id}", web::delete().to(delete_user_handler)),
    );
}
