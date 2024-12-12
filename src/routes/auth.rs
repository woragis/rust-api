use crate::{
    handlers::auth::auth::{login, register},
    handlers::auth::emails::{recover_password, verify_email},
    handlers::auth::profile::{delete_profile, read_profile, update_profile},
};
use actix_web::{
    web::{delete, get, post, put, scope},
    Scope,
};

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/register", post().to(register))
        .route("/login", post().to(login))
        .route("/recover-password", get().to(recover_password))
        .route("/verify", get().to(verify_email))
}

pub fn profile_routes() -> Scope {
    scope("/profile")
        .route("/", get().to(read_profile))
        .route("/", put().to(update_profile))
        .route("/", delete().to(delete_profile))
}
