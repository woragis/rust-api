//  // Live Stream Routes
//  // Projects: Twitch.tv and Kicks
//  pub static LIVE_STREAM_ROUTES: &str = "/live-stream/lives";
//  pub static LIVE_STREAM_SUBSCRIPTIONS_ROUTES: &str = "/live-stream/subscriptions";
//  pub static LIVE_STREAM_COMMENTS_ROUTES: &str = "/live-stream/comments";
//  pub static LIVE_STREAM_LIKES_ROUTES: &str = "/live-stream/likes";

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
