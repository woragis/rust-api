use crate::{
    config::routes::{
        NEWS_ARTICLES_ROUTES, NEWS_ARTICLES_TAGS_ROUTES, NEWS_COMMENTS_ROUTES, NEWS_LIKES_ROUTES,
        NEWS_TAGS_ROUTES, NEWS_VIEWS_TAGS_ROUTES,
    },
    handlers::news::articles::{
        create_article, delete_article, read_article, read_articles, update_article, update_article_status,
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
}

pub fn news_comments_routes() -> Scope {
    web::scope(NEWS_COMMENTS_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{comment_id}", web::get().to(read_article))
        .route("/{comment_id}", web::put().to(update_article))
        .route("/{comment_id}", web::delete().to(delete_article))
}

pub fn news_tags_routes() -> Scope {
    web::scope(NEWS_TAGS_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{tag_id}", web::get().to(read_article))
        .route("/{tag_id}", web::put().to(update_article))
        .route("/{tag_id}", web::delete().to(delete_article))
}

pub fn news_articles_tags_routes() -> Scope {
    web::scope(NEWS_ARTICLES_TAGS_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{article_tag_id}", web::get().to(read_article))
        .route("/{article_tag_id}", web::put().to(update_article))
        .route("/{article_tag_id}", web::delete().to(delete_article))
}

pub fn news_likes_routes() -> Scope {
    web::scope(NEWS_LIKES_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{like_id}", web::get().to(read_article))
        .route("/{like_id}", web::put().to(update_article))
        .route("/{like_id}", web::delete().to(delete_article))
}

pub fn news_views_routes() -> Scope {
    web::scope(NEWS_VIEWS_TAGS_ROUTES)
        .route("/", web::get().to(read_articles))
        .route("/", web::post().to(create_article))
        .route("/{view_id}", web::get().to(read_article))
        .route("/{view_id}", web::put().to(update_article))
        .route("/{view_id}", web::delete().to(delete_article))
}
