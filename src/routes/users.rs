use crate::handlers::users::{create_user, delete_user, read_user, read_users, update_user};
use actix_web::{web, Scope};

pub fn users_routes() -> Scope {
    web::scope("/users")
        .route("/", web::get().to(read_users))
        .route("/", web::post().to(create_user))
        .route("/{id}", web::get().to(read_user))
        .route("/{id}", web::put().to(update_user))
        .route("/{id}", web::delete().to(delete_user))
}
