use crate::{
    config::routes::{NEWS_ARTICLES_ROUTES, NEWS_TAGS_ROUTES},
    handlers::news::articles::{
        create_article, delete_article, read_article, read_articles, update_article,
        update_article_status,
    },
};
use actix_web::{web, Scope};

pub fn news_articles_routes() -> Scope {
    web::scope(NEWS_ARTICLES_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{article_id}", web::get().to(read_article))
        .route("/{article_id}", web::put().to(update_article))
        .route("/{article_id}/status", web::put().to(update_article_status))
        .route("/{article_id}", web::delete().to(delete_article))
        // comments
        .route("/{article_id}/comments", web::get().to(read_articles))
        .route(
            "/{article_id}/comments/{comment_id}",
            web::get().to(read_article),
        )
        .route(
            "/{article_id}/comments/{comment_id}",
            web::post().to(create_article),
        )
        .route(
            "/{article_id}/comments/{comment_id}",
            web::put().to(update_article),
        )
        .route(
            "/{article_id}/comments/{comment_id}",
            web::delete().to(delete_article),
        )
        // article tags
        .route("/{article_id}/tags", web::get().to(read_articles))
        .route("/{article_id}/tags/{tag_id}", web::put().to(update_article))
        .route(
            "/{article_id}/tags/{tag_id}",
            web::delete().to(delete_article),
        )
        // likes
        .route("/{article_id}/likes", web::get().to(read_articles))
        .route("/{article_id}/likes", web::post().to(create_article))
        // views
        .route("/{article_id}/views", web::get().to(read_articles))
        .route("/{article_id}/views", web::post().to(create_article))
}

pub fn news_tags_routes() -> Scope {
    web::scope(NEWS_TAGS_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{tag_id}", web::get().to(read_article))
        .route("/{tag_id}", web::put().to(update_article))
        .route("/{tag_id}", web::delete().to(delete_article))
}
