use crate::{
    config::routes::AUTH_ROUTES,
    handlers::auth::{login, profile, register},
};
use actix_web::{web, Scope};

pub fn auth_routes() -> Scope {
    web::scope(AUTH_ROUTES)
        .route("/login", web::post().to(login))
        .route("/register", web::post().to(register))
        .route("/profile", web::get().to(profile))
}
