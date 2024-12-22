use actix_web::{web::{scope, get, post, put, delete}, Scope};

use crate::handlers::blog::{posts::{create_post, delete_post, read_post, read_posts, update_post}, subscriptions::subscribe};

pub fn blog_posts_routes() -> Scope {
    scope("/blog/posts")
        .route("/", get().to(read_posts))
        .route("/", post().to(create_post))
        .route("/{post_id}", get().to(read_post))
        .route("/{post_id}", put().to(update_post))
        .route("/{post_id}", delete().to(delete_post))
}

pub fn blog_subscriptions_routes() -> Scope {
    scope("/blog/subscriptions")
        .route("/", post().to(subscribe))
}