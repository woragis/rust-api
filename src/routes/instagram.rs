//  // Instagram Social Media Routes
//  // Projects: Instagram
//  pub static INSTAGRAM_VIDEOS_ROUTES: &str = "/instagram/media";
//  pub static INSTAGRAM_COMMENTS_ROUTES: &str = "/instagram/comments";
//  pub static INSTAGRAM_LIKES_ROUTES: &str = "/instagram/likes";

use crate::{
    handlers::auth::auth::{login, register},
    handlers::auth::emails::{recover_password, verify_email},
    handlers::auth::profile::{delete_profile, read_profile, update_profile},
};
use actix_web::{web::{scope, post, get}, Scope};

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/register", post().to(register))
        .route("/login", post().to(login))
        .route("/recover-password", get().to(recover_password))
        .route("/verify", get().to(verify_email))
}
