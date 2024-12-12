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
use actix_web::{web::{scope, post, get}, Scope};

pub fn auth_routes() -> Scope {
    scope("/auth")
        .route("/register", post().to(register))
        .route("/login", post().to(login))
        .route("/recover-password", get().to(recover_password))
        .route("/verify", get().to(verify_email))
}
