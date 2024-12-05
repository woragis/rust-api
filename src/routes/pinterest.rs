//  // Pinterest Social Media Routes
//  // Projects: Pinterest
//  pub static PINTEREST_VIDEOS_ROUTES: &str = "/pinterest/media";
//  pub static PINTEREST_COMMENTS_ROUTES: &str = "/pinterest/comments";
//  pub static PINTEREST_LIKES_ROUTES: &str = "/pinterest/likes";

use crate::{
    handlers::auth::auth::{login, register},
    handlers::auth::emails::{recover_password, verify_email},
    handlers::auth::profile::{delete_profile, read_profile, update_profile},
};
use actix_web::{web, Scope};

pub fn auth_routes() -> Scope {
    web::scope("/auth")
        .route("/register", web::post().to(register))
        .route("/login", web::post().to(login))
        .route("/recover-password", web::get().to(recover_password))
        .route("/verify", web::get().to(verify_email))
}
