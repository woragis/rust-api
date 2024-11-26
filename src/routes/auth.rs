use crate::{
    config::routes::{AUTH_ROUTES, PROFILE_ROUTES},
    handlers::auth::{delete_profile, login, read_profile, register, update_profile},
};
use actix_web::{web, Scope};

pub fn auth_routes() -> Scope {
    web::scope(AUTH_ROUTES)
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
}

pub fn profile_routes() -> Scope {
    web::scope(PROFILE_ROUTES)
        .route("/", web::get().to(read_profile))
        .route("/", web::put().to(update_profile))
        .route("/", web::delete().to(delete_profile))
}
