use crate::handlers::users::{create_user, delete_user, read_user, read_users, update_user};
use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

pub fn users_routes() -> Scope {
    scope("/users")
        .route("/", get().to(read_users))
        .route("/", post().to(create_user))
        .route("/{user_id}", get().to(read_user))
        .route("/{user_id}", put().to(update_user))
        .route("/{user_id}", delete().to(delete_user))
}
