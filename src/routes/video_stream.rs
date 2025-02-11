//  // Video Stream Routes
//  // Projects: Youtube and Cos.tv
//  pub static VIDEO_STREAM_VIDEOS_ROUTES: &str = "/video-stream/videos";
//  pub static VIDEO_STREAM_COMMENTS_ROUTES: &str = "/video-stream/comments";
//  pub static VIDEO_STREAM_LIKES_ROUTES: &str = "/video-stream/likes";
//  pub static VIDEO_STREAM_SUBSCRIPTIONS_ROUTES: &str = "/video-stream/subscriptions";

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
