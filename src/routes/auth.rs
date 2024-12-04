use crate::{
    config::routes::{AUTH_ROUTES, PROFILE_ROUTES},
    handlers::auth::auth::{login, register},
    handlers::auth::emails::{recover_password, verify_email},
    handlers::auth::profile::{delete_profile, read_profile, update_profile},
};
use actix_web::{web, Scope};

pub fn auth_routes() -> Scope {
    web::scope(AUTH_ROUTES)
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/recover-password", web::get().to(recover_password))
        .route("/verify", web::get().to(verify_email))
}

pub fn profile_routes() -> Scope {
    web::scope(PROFILE_ROUTES)
        .route("/", web::get().to(read_profile))
        .route("/", web::put().to(update_profile))
        .route("/", web::delete().to(delete_profile))
}
