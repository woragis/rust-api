use crate::{
    config::routes::USERS_ROUTES,
    handlers::users::{create_user, delete_user, read_user, read_users, update_user},
};
use actix_web::{web, Scope};

pub fn users_routes() -> Scope {
    web::scope(USERS_ROUTES)
        .route("/", web::get().to(read_users))
        .route("/", web::post().to(create_user))
        .route("/{user_id}", web::get().to(read_user))
        .route("/{user_id}", web::put().to(update_user))
        .route("/{user_id}", web::delete().to(delete_user))
}
