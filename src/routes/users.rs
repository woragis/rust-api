use actix_web::web;
use crate::controllers::users::{
    create_user_handler, delete_user_handler, read_user_handler, read_users_handler,
    update_user_handler,
};

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
  cfg
            .route("/users", web::post().to(create_user_handler))
            .route("/users", web::get().to(read_users_handler))
            .route("/users/{id}", web::get().to(read_user_handler))
            .route("/users/{id}", web::put().to(update_user_handler))
            .route("/users/{id}", web::delete().to(delete_user_handler));
}